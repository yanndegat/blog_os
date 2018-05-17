#![feature(lang_items)] // required for defining the panic handler
#![feature(const_fn)]
#![no_std] // don't link the Rust standard library

extern crate spin;
extern crate volatile;
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
extern crate array_init;
#[cfg(test)]
extern crate std;

extern crate x86_64;
#[macro_use]
extern crate bitflags;

pub mod console;
