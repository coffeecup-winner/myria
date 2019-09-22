#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop { }
}

#[no_mangle]
pub extern "C" fn _entry() -> ! {
    let vga = 0xb8000 as *mut u32;
    unsafe {
        *vga = 0x07690748;
    }
    loop { }
}
