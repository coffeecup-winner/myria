#![feature(format_args_nl)]
#![feature(panic_info_message)]

#![no_std]

use core::panic::PanicInfo;

mod gdt;
mod idt;
mod vga;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("Kernel panic: ");
    if let Some(message) = info.message() {
        vga::_print(*message);
        println!();
    }
    loop { }
}

#[no_mangle]
pub extern "C" fn _entry() -> ! {
    vga::clear_screen();
    println!("Booting Myria");
    gdt::load_gdt();
    println!("Loaded GDT");
    idt::load_idt();
    println!("Loaded IDT");
    panic!("Halt");
}
