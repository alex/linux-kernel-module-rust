use core::fmt;

use types::c_int;

pub struct KernelConsole;

extern "C" {
    fn printk_helper(s: *const u8, len: c_int) -> c_int;
}

pub fn printk(s: &[u8]) {
    // TODO: I believe printk never fails
    unsafe { printk_helper(s.as_ptr(), s.len() as c_int) };
}

// From kernel/print/printk.c
const LOG_LINE_MAX: usize = 1024 - 32;

pub struct LogLineWriter {
    data: [u8; LOG_LINE_MAX],
    pos: usize,
}

impl LogLineWriter {
    pub fn new() -> LogLineWriter {
        LogLineWriter {
            data: [0u8; LOG_LINE_MAX],
            pos: 0,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        return &self.data[..self.pos];
    }
}

impl fmt::Write for LogLineWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let copy_len = if LOG_LINE_MAX - self.pos >= s.as_bytes().len() {
            s.as_bytes().len()
        } else {
            LOG_LINE_MAX - self.pos
        };
        self.data[self.pos..self.pos + copy_len].copy_from_slice(&s.as_bytes()[..copy_len]);
        self.pos += copy_len;
        return Ok(());
    }
}

#[macro_export]
macro_rules! println {
    () => ({
        $crate::printk::printk("\x016\n".as_bytes());
    });
    ($fmt:expr) => ({
        $crate::printk::printk(concat!("\x016", $fmt, "\n").as_bytes());
    });
    ($fmt:expr, $($arg:tt)*) => ({
        use ::core::fmt::{self, Write};
        let mut writer = $crate::printk::LogLineWriter::new();
        // TODO: Don't allocate!
        let _ = fmt::write(&mut writer, format_args!(concat!("\x016", $fmt, "\n"), $($arg)*)).unwrap();
        $crate::printk::printk(writer.as_bytes());
    });
}
