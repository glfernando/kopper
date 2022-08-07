use crate::lib::print::get_stdio;
use core::fmt::LowerHex;

/// Hexdump configuration struct
///
/// Use to pass additional information to hexdump functions
///
/// # Examples
///
/// ```
/// let config = HexdumpConfig::new();
/// config.group(4).ascii(true).offset(true);
/// hexdump
pub struct HexdumpConfig {
    group: usize,
    offset: bool,
    ascii: bool,
}

/// ```
impl Default for HexdumpConfig {
    fn default() -> Self {
        Self {
            group: 4,
            offset: false,
            ascii: false,
        }
    }
}

impl HexdumpConfig {
    /// Construne new HexdumpConfig
    ///
    /// It uses the default parameters
    pub fn new() -> Self {
        Default::default()
    }

    /// Changes group value
    pub fn group(&mut self, group: usize) -> &mut Self {
        self.group = group;
        self
    }

    /// Changes offset value
    pub fn offset(&mut self, offset: bool) -> &mut Self {
        self.offset = offset;
        self
    }

    /// Changes ascii value
    pub fn ascii(&mut self, ascii: bool) -> &mut Self {
        self.ascii = ascii;
        self
    }

    /// calls hexdump_config using this instance as the config value.
    pub fn hexdump(&self, out: &mut impl core::fmt::Write, buf: impl AsRef<[u8]>) {
        hexdump_u8(out, buf.as_ref(), &self);
    }
}

fn print_slice<T: LowerHex>(out: &mut impl core::fmt::Write, data: &[T]) {
    for (i, x) in data.iter().enumerate() {
        if i != 0 {
            write!(out, " ").unwrap();
        }
        let width = core::mem::size_of::<T>() * 2;
        write!(out, "{x:0width$x}").unwrap();
    }
}

fn hexdump_u8(out: &mut impl core::fmt::Write, buf: &[u8], config: &HexdumpConfig) {
    let &HexdumpConfig {
        group,
        offset: print_offset,
        ascii,
    } = config;
    let mut rem = buf.len();
    let mut offset = 0usize;

    while rem != 0 {
        let addr = &buf[offset] as *const u8 as usize;

        if print_offset {
            write!(out, "{:x}: ", addr).unwrap();
        }

        let len = core::cmp::min(rem, 16);
        let count = len / group;

        match group {
            1 => print_slice(out, &buf[offset..offset + len]),
            2 => {
                let p = addr as *const u16;
                let buf = unsafe { core::slice::from_raw_parts(p, count) };
                print_slice(out, buf);
            }
            4 => {
                let p = addr as *const u32;
                let buf = unsafe { core::slice::from_raw_parts(p, count) };
                print_slice(out, buf);
            }
            8 => {
                let p = addr as *const u64;
                let buf = unsafe { core::slice::from_raw_parts(p, count) };
                print_slice(out, buf);
            }
            _ => panic!(),
        }

        let group_rem = len % group;
        if group_rem != 0 {
            write!(out, " ").unwrap(); // separator
            let spaces = group - group_rem;
            for _ in 0..spaces {
                write!(out, "  ").unwrap(); // each char uses 2 chars
            }

            let offset = offset + count * group;
            for x in buf[offset..offset + group_rem].iter().rev() {
                write!(out, "{x:02x}").unwrap();
            }
        }

        if ascii {
            write!(out, " ").unwrap();

            for x in &buf[offset..offset + len] {
                let c = *x as char;
                write!(out, "{}", if c.is_ascii_graphic() { c } else { '.' }).unwrap();
            }
        }

        writeln!(out).unwrap();

        offset += len;
        rem -= len;
    }
}

/// Hexdump to `out` of the input `buf` buffer with configuration from `config`
///
/// # Examples
///
/// ```
/// use hexdump::{self, HexdumpConfig};
///
/// let config = HexdumpConfig::new();
/// config.ascii(true).offset(true);
///
///  hexdump_config(out, buf, &config);
///
/// ```
pub fn hexdump_config(
    out: &mut impl core::fmt::Write,
    buf: impl AsRef<[u8]>,
    config: &HexdumpConfig,
) {
    hexdump_u8(out, buf.as_ref(), config);
}

/// Hexudmp buffer to standard output using default configuration.
pub fn hexdump(buf: impl AsRef<[u8]>) {
    hexdump_config(&mut get_stdio(), buf, &Default::default());
}
