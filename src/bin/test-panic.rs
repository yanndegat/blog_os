#![feature(lang_items)] // required for defining the panic handler
#![feature(const_fn)]
#![no_std] // don't link the Rust standard library
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points

#[macro_use]
extern crate blog_os;

extern crate x86_64;

/// This function is the entry point, since the linker looks for a function
/// named `_start_` by default.
#[cfg(not(test))]
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    panic!("bla");

    let mut port = x86_64::instructions::port::Port::<u8>::new(0xf4);
    unsafe { port.write(0u8) };

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_panic(
    msg: core::fmt::Arguments,
    file: &'static str,
    line: u32,
    column: u32,
) -> ! {
    println!("not ok");

    println!("panic: {} at {}:{}:{}", msg, file, line, column);

    let mut port = x86_64::instructions::port::Port::<u8>::new(0xf4);
    unsafe { port.write(0u8) };

    loop {}
}
