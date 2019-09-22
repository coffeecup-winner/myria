#![feature(panic_info_message)]

#![no_std]

use core::panic::PanicInfo;

mod vga;

use vga::*;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut vga = VGA::new(80, 25);
    vga.set_cursor(0, 0);
    core::fmt::write(&mut vga, format_args!("Kernel panic: "));
    if let Some(message) = info.message() {
        core::fmt::write(&mut vga, *message);
    }
    loop { }
}

#[no_mangle]
pub extern "C" fn _entry() -> ! {
    let vga = VGA::new(80, 25);
    for y in 0..vga.height() {
        for x in 0..vga.width() {
            vga.putc(x, y, '*');
        }
    }
    panic!("Halt");
}
