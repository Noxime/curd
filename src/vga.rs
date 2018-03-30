// This provides basic *x86-64* specific VGA text mode printing

// We don't want the compiler to optimize away `buffer` since we never read from it
// using volatile prevents that optimization
use volatile::Volatile;
use spin::Mutex;

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Color {
    Black      = 0x0,
    Blue       = 0x1,
    Green      = 0x2,
    Cyan       = 0x3,
    Red        = 0x4,
    Magenta    = 0x5,
    Brown      = 0x6,
    LightGray  = 0x7,
    DarkGray   = 0x8,
    LightBlue  = 0x9,
    LightGreen = 0xA,
    LightCyan  = 0xB,
    LightRed   = 0xC,
    Pink       = 0xD,
    Yellow     = 0xE,
    White      = 0xF,
}

// Our screen buffer, 80x25
struct Buffer {
    chars: [[Volatile<u16>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        buffer: unsafe { &mut *(0xB8000 as *mut Buffer) },
    });
}
    

// This holds our writer variables
pub struct Writer {
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn pixel(&mut self, x: usize, y: usize, v: Color) {
        self.buffer.chars[y][x].write((v as u16) << 12 | 0x20);
    }

    fn clear(&mut self) {
        for x in 0 .. BUFFER_WIDTH {
            for y in 0 .. BUFFER_HEIGHT {
                self.pixel(x, y, Color::DarkGray);
            }
        }
    }
}

pub fn init() {
    WRITER.lock().clear();
}