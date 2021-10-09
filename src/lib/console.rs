extern crate alloc;

use crate::device;
use crate::{println, shell_cmd};
use alloc::collections::vec_deque::VecDeque;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;
use core::fmt::Write;
use core::write;

const MAX_LINE_SIZE: usize = 128;
const MAX_HISTORY_SIZE: usize = 16;

const PROMPT: &'static str = "ksh> ";
const BACKSPACE: char = '\x08';
const DEL: char = '\x7f';
const TAB: char = '\t';
const BELL: char = '\x07';
const ESC: char = '\x1b';
const CUR_BACK: &'static str = "\x1b[D";
const CUR_FORWARD: &'static str = "\x1b[C";
const ERASE_EOL: &'static str = "\x1b[K";

/// Console abstraction
pub struct Console<'a> {
    con: &'a (dyn device::console::Console + 'a),
}

impl<'a> Console<'a> {
    /// create new Console instance
    pub fn new(con: &'a impl device::console::Console) -> Self {
        Self { con }
    }

    /// send a char to the console
    pub fn putc(&self, c: char) -> Result<(), &'static str> {
        self.con.putc(c)
    }

    /// send a &str to the console
    pub fn puts(&self, s: &str) -> Result<(), &'static str> {
        for c in s.chars() {
            self.con.putc(c)?;
        }
        Ok(())
    }

    /// read a char from the console
    pub fn getc(&self) -> Result<char, &'static str> {
        self.con.getc()
    }
}

impl<'a> core::fmt::Write for Console<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.con.putc(c).map_err(|_| fmt::Error {})?;
        }
        Ok(())
    }
}

#[doc(hidden)]
pub fn _cprint(con: &mut Console, args: fmt::Arguments) {
    con.write_fmt(args).unwrap();
}

/// print to a specific console
#[macro_export]
macro_rules! cprint {
    ($con:expr, $($arg:tt)*) => ({
        $crate::lib::console::_cprint($con, format_args_nl!($($arg)*));
    })
}

/// print to a specific console
#[macro_export]
macro_rules! cprintln {
    ($con:expr) => ($crate::cprint!($con, "\n"));
    ($con:expr, $($arg:tt)*) => ({
        $crate::lib::console::_cprint($con, format_args_nl!($($arg)*));
    })
}

/// run interactive shell
pub fn run_shell(dev_con: &impl device::console::Console) -> Result<(), &'static str> {
    let mut con = Console::new(dev_con);

    let mut history = VecDeque::new();

    loop {
        con.puts(PROMPT)?;
        let mut line = if MAX_HISTORY_SIZE == history.len() {
            history.pop_front().unwrap()
        } else {
            String::new()
        };
        get_line(&mut con, &mut line, &history)?;

        if let Err(e) = run_cmd(&mut con, &line) {
            println!("{}", e);
        } else {
            if !line.is_empty() {
                // save command (if different from previous one)
                if history.len() == 0 || !line.eq(&history[history.len() - 1]) {
                    history.push_back(line);
                }
            }
        }
    }
}

fn get_line(
    con: &mut Console,
    line: &mut String,
    history: &VecDeque<String>,
) -> Result<(), &'static str> {
    line.clear();
    let mut pos = 0usize;
    let mut hi = history.len();
    let mut line_bak = String::new();
    loop {
        let c = con.getc()?;
        let size = line.len();

        match c {
            '\n' | '\r' => {
                con.puts("\n")?;
                break;
            }
            BACKSPACE | DEL => {
                if pos == 0 {
                    con.putc(BELL)?;
                    continue;
                }
                con.puts(CUR_BACK)?;
                pos -= 1;
                line.remove(pos);
                con.puts(&line[pos..])?;
                con.puts(ERASE_EOL)?;
                set_cursor_pos(con, pos)?;
            }
            ESC => match get_esc_seq(con)? {
                EscSeq::CurUp => {
                    if hi == 0 {
                        con.putc(BELL)?;
                        continue;
                    }
                    if hi == history.len() {
                        line_bak = line.clone();
                    }
                    hi -= 1;
                    line.clear();
                    line.push_str(&history[hi][..]);
                    set_cursor_pos(con, 0)?;
                    con.puts(line)?;
                    con.puts(ERASE_EOL)?;
                    pos = line.len();
                }
                EscSeq::CurDown => {
                    if hi == history.len() {
                        con.putc(BELL)?;
                        continue;
                    }
                    hi += 1;
                    line.clear();
                    if hi == history.len() {
                        line.push_str(&line_bak);
                    } else {
                        line.push_str(&history[hi]);
                    }
                    set_cursor_pos(con, 0)?;
                    con.puts(line)?;
                    con.puts(ERASE_EOL)?;
                    pos = line.len();
                }
                EscSeq::CurForward => {
                    if pos == size {
                        con.putc(BELL)?;
                        continue;
                    }
                    con.puts(CUR_FORWARD)?;
                    pos += 1;
                }
                EscSeq::CurBackward => {
                    if pos == 0 {
                        con.putc(BELL)?;
                        continue;
                    }
                    con.puts(CUR_BACK)?;
                    pos -= 1;
                }
                EscSeq::CurHome => {
                    pos = 0;
                    set_cursor_pos(con, pos)?;
                }
                EscSeq::CurEnd => {
                    pos = size;
                    set_cursor_pos(con, pos)?;
                }
                EscSeq::DEL => {
                    if pos == size {
                        con.putc(BELL)?;
                        continue;
                    }
                    line.remove(pos);
                    con.puts(&line[pos..])?;
                    con.puts(ERASE_EOL)?;
                    set_cursor_pos(con, pos)?;
                }
                _ => continue,
            },
            TAB => {
                if pos == 0 || pos != line.len() {
                    con.putc(BELL)?;
                    continue;
                }
                let matches = find_all_matches(&line);
                let matches_len = matches.len();
                if matches_len == 0 {
                    con.putc(BELL)?;
                    continue;
                }
                if matches_len == 1 {
                    // lets autoocmplete the command
                    con.puts(&matches[0][pos..])?;
                    line.push_str(&matches[0][pos..]);
                    // add extra space
                    con.putc(' ')?;
                    line.push(' ');
                    pos = line.len();
                    continue;
                }

                // multiple matches
                let mci = find_mci(&matches);
                if pos < mci {
                    // update command part to all common part
                    con.puts(&matches[0][pos..mci])?;
                    line.push_str(&matches[0][pos..mci]);
                    pos = line.len();
                    continue;
                }

                // let's print them all
                con.putc('\n')?;
                matches.iter().for_each(|&cmd| cprint!(con, "{}", cmd));
                // restore promp
                con.puts(PROMPT)?;
                con.puts(line)?;
            }
            c => {
                // check if we cannot accept more chars
                if size == MAX_LINE_SIZE - 1 {
                    con.putc(BELL)?;
                    continue;
                }
                if pos == size {
                    con.putc(c)?;
                    line.push(c);
                    pos += 1;
                    continue;
                }
                // we are insertint chars, so move other data
                con.putc(c)?;
                line.insert(pos, c);
                pos += 1;
                con.puts(&line[pos..])?;
                set_cursor_pos(con, pos)?;
            }
        }
    }
    Ok(())
}

fn set_cursor_pos(con: &mut Console, pos: usize) -> Result<(), &'static str> {
    // move position to 0
    con.putc('\r')?;
    write!(con, "\x1b[{}C", PROMPT.len() + pos).map_err(|_| "error to format cursor position")?;

    Ok(())
}

enum EscSeq {
    CurUp,
    CurDown,
    CurForward,
    CurBackward,
    CurHome,
    CurEnd,
    DEL,
    NONE,
}

fn get_esc_seq(con: &mut Console) -> Result<EscSeq, &'static str> {
    let esc = match con.getc()? {
        '[' => match con.getc()? {
            'A' => EscSeq::CurUp,
            'B' => EscSeq::CurDown,
            'C' => EscSeq::CurForward,
            'D' => EscSeq::CurBackward,
            'H' => EscSeq::CurHome,
            'F' => EscSeq::CurEnd,
            '1' => match con.getc()? {
                '~' => EscSeq::CurHome,
                _ => EscSeq::NONE,
            },
            '3' => match con.getc()? {
                '~' => EscSeq::DEL,
                _ => EscSeq::NONE,
            },
            _ => EscSeq::NONE,
        },
        'O' => match con.getc()? {
            'F' => EscSeq::CurEnd,
            _ => EscSeq::NONE,
        },
        _ => EscSeq::NONE,
    };
    Ok(esc)
}

/// console arguments
pub type Args<'a> = core::str::SplitWhitespace<'a>;

#[doc(hidden)]
pub struct ConCmd {
    name: &'static str,
    help: &'static str,
    func: fn(&mut Console, Args) -> core::result::Result<(), &'static str>,
}

impl ConCmd {
    /// create new shell command
    pub const fn new(
        name: &'static str,
        help: &'static str,
        func: fn(&mut Console, Args) -> Result<(), &'static str>,
    ) -> Self {
        Self { name, help, func }
    }
}

extern "Rust" {
    static __shell_cmds_start: ConCmd;
    static __shell_cmds_end: ConCmd;
}

fn get_cmd_list() -> &'static [ConCmd] {
    use core::mem;

    let cmds_start = unsafe { &__shell_cmds_start as *const _ as usize };
    let cmds_end = unsafe { &__shell_cmds_end as *const _ as usize };
    let size = (cmds_end - cmds_start) / mem::size_of::<ConCmd>();
    let cmds = unsafe { core::slice::from_raw_parts(cmds_start as *const ConCmd, size) };

    cmds
}

fn find_all_matches(line: &str) -> Vec<&'static str> {
    let cmds = get_cmd_list();

    cmds.iter()
        .filter_map(|cmd| {
            if cmd.name.starts_with(line) {
                Some(cmd.name)
            } else {
                None
            }
        })
        .collect()
}

fn find_mci(cmds: &Vec<&str>) -> usize {
    let name = cmds[0];
    for i in 1..name.len() {
        for &cmd in cmds {
            if name.as_bytes()[i] != cmd.as_bytes()[i] {
                return i;
            }
        }
    }
    name.len()
}

/// run a shell command
pub fn run_cmd(con: &mut Console, cmd_str: &str) -> Result<(), &'static str> {
    // get command name or exit if empty.
    let name = match cmd_str.split_whitespace().next() {
        Some(name) => name,
        None => return Ok(()), //empty command
    };

    let cmds = get_cmd_list();

    for cmd in cmds {
        if cmd.name == name {
            return (cmd.func)(con, cmd_str.split_whitespace());
        }
    }

    Err("command not found")
}

#[shell_cmd(help, "list all commands + description")]
fn help_cmd(con: &mut Console, args: Args) -> Result<(), &'static str> {
    let name = args.into_iter().skip(1).next().unwrap_or("");

    let cmds = get_cmd_list();
    for cmd in cmds {
        if cmd.name.starts_with(name) {
            cprintln!(con, "{:<16} - {}", cmd.name, cmd.help);
        }
    }

    Ok(())
}
