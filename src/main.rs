#![feature(lang_items)] // Nightly allows us to use this
#![no_std] // We do not have a freestanding std implementation
#![no_main] // We have a custom entrypoint
#![feature(const_fn)] // So we can have static initializers

// external crates
extern crate rlibc;
extern crate volatile;
#[macro_use]
extern crate lazy_static;
extern crate spin;
extern crate x86_64;

// module imports
#[macro_use]
mod serial;
mod vga;


// Kernel entry point
#[no_mangle]
pub fn _start() -> ! {
    println!("Booting Curd kernel");
    println!("Serial console enabled");
    vga::init();
    println!("VGA buffer cleared");

    loop {}

    panic!("Kernel ran out of code");
}

// Panic implementation that just locks the thread in spin
#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(msg: core::fmt::Arguments,
    file: &'static str, line: u32, column: u32) -> !
{
    println!(">>> PANIC! {}:{}:{} <<<", file, line, column);
    println!("{}", msg);
    loop {}
}

