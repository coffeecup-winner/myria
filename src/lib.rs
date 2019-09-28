#![feature(format_args_nl)]
#![feature(panic_info_message)]

#![no_std]

#[macro_use]
extern crate lazy_static;

use core::panic::PanicInfo;

#[macro_use]
mod vga;

mod gdt;
mod idt;
mod pic;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("Kernel panic: ");
    if let Some(message) = info.message() {
        vga::_print(*message);
        println!();
    }
    loop {
        unsafe {
            x86::halt();
        }
    }
}

#[no_mangle]
pub extern "C" fn _entry() -> ! {
    vga::clear_screen();
    println!("Booting Myria");
    gdt::load_gdt();
    println!("Loaded GDT");
    pic::remap(0x20, 0x28);
    println!("Remapped PIC");
    idt::load_idt();
    println!("Loaded IDT");
    panic!("Halt");
}
