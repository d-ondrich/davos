#![no_std]
#![no_main]

// needed to link rlibc since functions not called directly
extern crate rlibc;

mod vga_buffer;
use core::panic::PanicInfo;

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}

