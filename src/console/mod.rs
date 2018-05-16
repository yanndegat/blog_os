#[macro_use]
pub mod vga_buffer;
pub mod serial;
mod uart_16550;


/// Like the `print!` macro in the standard library, but prints to the VGA text buffer.
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::console::vga_buffer::print(format_args!($($arg)*));
        $crate::console::serial::print(format_args!($($arg)*));
    };
}

/// Like the `print!` macro in the standard library, but prints to the VGA text buffer.
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}
