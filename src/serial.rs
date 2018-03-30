// A very simple serial interface for debugging the kernel

use spin::Mutex;
use core::fmt;

use x86_64::instructions::port::{outb, inb};

#[allow(dead_code)]
#[repr(u16)]
#[derive(Copy, Clone)]
pub enum Port {
    COM1 = 0x3F8,
    COM2 = 0x2F8,
    COM3 = 0x3E8,
    COM4 = 0x2E8,
}

pub struct Writer {
    port: Port
}

impl Writer {
    // check if transmit is empty, so we can write
    fn transmit_empty(&self) -> bool {
        unsafe { (inb(self.port as u16 + 5) & 0x20) == 0 }
    }

    // have we received something
    fn received(&self) -> bool {
        unsafe { (inb(self.port as u16 + 5) & 0x01) == 0 }
    } 

    // write a single byte to output
    fn write_b(&self, v: u8) {
        // check if we can write right now
        while self.transmit_empty() {}
        unsafe { outb(self.port as u16, v) }
    }

    fn write(&self, s: &str) {
        for b in s.bytes() {
            self.write_b(b);
        }
    }

    fn read(&self) -> char {
        while self.received() {}
        unsafe { inb(self.port as u16) as char }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = {
        let port = Port::COM1;

        // set up serial communication
        unsafe {
            outb(port as u16 + 1, 0x00); // Disable interrupts
            outb(port as u16 + 3, 0x80); // Enable DLAB
            outb(port as u16 + 0, 0x03); // Rate divisor lo byte (38400)
            outb(port as u16 + 1, 0x00); // Rate divisor hi byte
            outb(port as u16 + 3, 0x03); // 8 bits, no parity, one stop bit
            outb(port as u16 + 2, 0xC7); // Enable FIFO, clear, 14 byte threshold
            outb(port as u16 + 4, 0x0B); // IRQs enabled, RTS/DSR set
        }

        
        Mutex::new(Writer { port })
    };
}

#[allow(unused_macros)]
macro_rules! print {
    ($($arg:tt)*) => ($crate::serial::print(format_args!($($arg)*)));
}

#[allow(unused_macros)]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[allow(unused_macros)]
macro_rules! read {
    () => ($crate::serial::read());
}

pub fn read() -> char {
    WRITER.lock().read()
}