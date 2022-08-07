extern crate alloc;
extern crate num_traits;

use alloc::vec::Vec;
use kopper::lib::console::{Args, ConCmd, Console};
use kopper::lib::hexdump::HexdumpConfig;
use kopper::{cprintln, shell_cmd};
use num_traits::Num;

fn mem_usage(con: &mut Console) {
    cprintln!(con, "mem <[0x]hex_addr>[:size|..<[0x]hex_range> [value]");
}

fn parse_hex<T: Num>(s: &str) -> Result<T, &'static str> {
    T::from_str_radix(s.trim_start_matches("0x"), 16).map_err(|_| "Failed to parse hex integer")
}

fn parse_num<T: Num>(s: &str) -> Result<T, &'static str> {
    let mut base = 10;
    let mut offset = 0;
    if s.starts_with("0x") {
        offset = 2;
        base = 16;
    } else if s.starts_with("0o") {
        offset = 2;
        base = 8;
    } else if s.starts_with("0b") {
        offset = 2;
        base = 2;
    }

    T::from_str_radix(&s[offset..], base).map_err(|_| "Failed to parse integer")
}

#[shell_cmd(mem, "read and write memory")]
pub fn mem_cmd(con: &mut Console, args: Args) -> Result<(), &'static str> {
    let mut args: Vec<&str> = args.into_iter().skip(1).collect();

    let mut group = 4;
    let mut print_offset = false;
    let mut ascii = false;

    // parse command options
    {
        let mut i = 0usize;
        while i < args.len() {
            let opt = args[i];
            match opt {
                "-g" => {
                    // remove option
                    args.remove(i);
                    // we need another parameter for core number
                    if i >= args.len() {
                        cprintln!(con, "missing group number");
                    }
                    group = args[i].parse().map_err(|e| {
                        cprintln!(con, "failde to parse group number {}", e);
                        "parse error"
                    })?;
                    args.remove(i);
                }
                "-o" => {
                    // remove option
                    args.remove(i);
                    print_offset = true;
                }
                "-a" => {
                    // remove option
                    args.remove(i);
                    ascii = true;
                }
                _ => i += 1,
            }
        }
    }

    // validate group
    if group == 0 || ((group - 1) & group) != 0 || group > 8 {
        cprintln!(con, "invalid group number {}", group);
        return Err("invalid params");
    }

    if args.len() < 1 || args.len() > 2 {
        mem_usage(con);
        return Ok(());
    }

    let (addr, size) = match args[0].split_once(':') {
        Some((addr, size)) => (addr, parse_num::<usize>(size)?),
        None => match args[0].split_once("..") {
            Some((addr, end)) => {
                let start: usize = parse_hex::<usize>(addr)?;
                let end: usize = parse_hex::<usize>(end)?;
                if start > end {
                    return Err("invalid address range");
                }
                (addr, end - start)
            }
            None => (args[0], 4),
        },
    };
    let addr: usize = parse_hex::<usize>(addr)?;

    if args.len() == 2 {
        match size {
            1 => {
                let val: u8 = parse_num(args[1])?;
                let p = addr as *mut u8;
                unsafe {
                    p.write_volatile(val);
                }
            }
            2 => {
                let val: u16 = parse_num(args[1])?;
                let p = addr as *mut u16;
                unsafe {
                    p.write_volatile(val);
                }
            }
            4 => {
                let val: u32 = parse_num(args[1])?;
                let p = addr as *mut u32;
                unsafe {
                    p.write_volatile(val);
                }
            }
            8 => {
                let val: u64 = parse_num(args[1])?;
                let p = addr as *mut u64;
                unsafe {
                    p.write_volatile(val);
                }
            }
            x => {
                cprintln!(con, "unsupported writting size {}", x);
                return Err("invalid params");
            }
        }
        return Ok(());
    } else {
        match size {
            1 => {
                let p = addr as *const u8;
                let val = unsafe { p.read_volatile() };
                cprintln!(con, "{:02x}", val);
            }
            2 => {
                let p = addr as *const u16;
                let val = unsafe { p.read_volatile() };
                cprintln!(con, "{:04x}", val);
            }
            4 => {
                let p = addr as *const u32;
                let val = unsafe { p.read_volatile() };
                cprintln!(con, "{:08x}", val);
            }
            8 => {
                let p = addr as *const u64;
                let val = unsafe { p.read_volatile() };
                cprintln!(con, "{:016x}", val);
            }
            _ => {
                let p = addr as *const u8;
                let buf = unsafe { core::slice::from_raw_parts(p, size) };
                HexdumpConfig::new()
                    .group(group)
                    .offset(print_offset)
                    .ascii(ascii)
                    .hexdump(con, buf);
            }
        }
    }
    Ok(())
}
