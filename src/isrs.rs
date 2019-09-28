use crate::pic;
use x86::io;

#[repr(C)]
#[repr(packed)]
pub struct ExceptionFrame {
    // segment registers
    gs: u32,
    fs: u32,
    es: u32,
    ds: u32,
    // pushad
    edi: u32,
    esi: u32,
    ebp: u32,
    esp: u32,
    ebx: u32,
    edx: u32,
    ecx: u32,
    eax: u32,
    // Custom data
    exception_number: u32,
    error_code: u32,
    // CPU data
    eip: u32,
    cs: u32,
    eflags: u32,
    user_esp: u32,
    ss: u32,
}

#[no_mangle]
pub unsafe extern "C" fn exception_handler(frame: &ExceptionFrame) {
    print!("[EXC] Got exception ");
    match frame.exception_number {
        13 => {
            print!("#GP");
            if frame.error_code != 0 {
                let external = frame.error_code & 0x1 == 1;
                let table = match (frame.error_code >> 1) & 0x11 {
                    0x0 => "GDT",
                    0x1 | 0x3 => "IDT",
                    0x2 => "LDT",
                    _ => panic!("Programmer error"),
                };
                let index = (frame.error_code >> 3) & 0x1fff;
                println!(": {} idx {} ({})", table, index, if external { "external" } else { "internal" });
            } else {
                println!();
            }
        }
        _ => {
            println!("{} (0x{:x})", frame.exception_number, frame.error_code);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn irq0_handler() {
    // Timer
    io::outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq1_handler() {
    // Keyboard
    let scancode = io::inb(0x60);
    println!("[IRQ] Got keyboard event, scancode 0x{:x}", scancode);
    io::outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq2_handler() {
    println!("[IRQ] Got IRQ2");
    io::outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq3_handler() {
    println!("[IRQ] Got IRQ3");
    io::outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq4_handler() {
    println!("[IRQ] Got IRQ4");
    io::outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq5_handler() {
    println!("[IRQ] Got IRQ5");
    io::outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq6_handler() {
    println!("[IRQ] Got IRQ6");
    io::outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq7_handler() {
    println!("[IRQ] Got IRQ7");
    io::outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq8_handler() {
    println!("[IRQ] Got IRQ8");
    io::outb(pic::PIC1_CMD, pic::PIC_CMD_EOI);
    io::outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq9_handler() {
    println!("[IRQ] Got IRQ9");
    io::outb(pic::PIC1_CMD, pic::PIC_CMD_EOI);
    io::outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq10_handler() {
    println!("[IRQ] Got IRQ10");
    io::outb(pic::PIC1_CMD, pic::PIC_CMD_EOI);
    io::outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq11_handler() {
    println!("[IRQ] Got IRQ11");
    io::outb(pic::PIC1_CMD, pic::PIC_CMD_EOI);
    io::outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq12_handler() {
    println!("[IRQ] Got IRQ12");
    io::outb(pic::PIC1_CMD, pic::PIC_CMD_EOI);
    io::outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq13_handler() {
    println!("[IRQ] Got IRQ13");
    io::outb(pic::PIC1_CMD, pic::PIC_CMD_EOI);
    io::outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq14_handler() {
    println!("[IRQ] Got IRQ14");
    io::outb(pic::PIC1_CMD, pic::PIC_CMD_EOI);
    io::outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq15_handler() {
    println!("[IRQ] Got IRQ15");
    io::outb(pic::PIC1_CMD, pic::PIC_CMD_EOI);
    io::outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}
