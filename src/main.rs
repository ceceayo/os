
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(jes_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate volatile;
extern crate spin;
extern crate x86_64;
extern crate uart_16550;
extern crate lazy_static;
extern crate jes_os;


use core::panic::PanicInfo;
use jes_os::{serial_println, println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("[     ] Hello World{}", "!");
    println!("[ ans ] The answer is {}!", 42);

    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    jes_os::test_panic_handler(info)
}