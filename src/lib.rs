#![no_std]
#![no_main]

mod display;

use crate::display::Display;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello world\n";

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    Display::new(vga_buffer).print(HELLO);

    loop {}
}