use super::uart_16550::SerialPort;

pub static mut COM1: SerialPort = SerialPort::new(0x3F8);
pub static mut COM2: SerialPort = SerialPort::new(0x2F8);

pub unsafe fn init() {
    COM1.init();
    COM2.init();
}

pub fn print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        let _ = COM1.write_fmt(args);
    }
}
