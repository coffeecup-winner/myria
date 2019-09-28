use crate::pic;
use x86::io;

#[repr(C)]
#[repr(packed)]
struct IDTDescriptor {
    offset0: u16,
    selector: u16,
    zero: u8,
    type_attr: u8,
    offset1: u16,
}

#[repr(C)]
#[repr(packed)]
struct IDTArray {
    limit: u16,
    base: u32,
}

extern "C" {
    fn _load_idt(idta: *const IDTArray);
    fn _exc0();
    fn _exc1();
    fn _exc2();
    fn _exc3();
    fn _exc4();
    fn _exc5();
    fn _exc6();
    fn _exc7();
    fn _exc8();
    fn _exc9();
    fn _exc10();
    fn _exc11();
    fn _exc12();
    fn _exc13();
    fn _exc14();
    fn _exc15();
    fn _exc16();
    fn _exc17();
    fn _exc18();
    fn _exc19();
    fn _exc20();
    fn _exc21();
    fn _exc22();
    fn _exc23();
    fn _exc24();
    fn _exc25();
    fn _exc26();
    fn _exc27();
    fn _exc28();
    fn _exc29();
    fn _exc30();
    fn _exc31();
    fn _irq0();
    fn _irq1();
    fn _irq2();
    fn _irq3();
    fn _irq4();
    fn _irq5();
    fn _irq6();
    fn _irq7();
    fn _irq8();
    fn _irq9();
    fn _irq10();
    fn _irq11();
    fn _irq12();
    fn _irq13();
    fn _irq14();
    fn _irq15();
}

lazy_static! {
    static ref IDT: [IDTDescriptor; 256] = [
        // 0x01..0x20 - CPU exceptions
        IDTDescriptor::interrupt(_exc0 as u32),
        IDTDescriptor::interrupt(_exc1 as u32),
        IDTDescriptor::interrupt(_exc2 as u32),
        IDTDescriptor::interrupt(_exc3 as u32),
        IDTDescriptor::interrupt(_exc4 as u32),
        IDTDescriptor::interrupt(_exc5 as u32),
        IDTDescriptor::interrupt(_exc6 as u32),
        IDTDescriptor::interrupt(_exc7 as u32),
        IDTDescriptor::interrupt(_exc8 as u32),
        IDTDescriptor::interrupt(_exc9 as u32),
        IDTDescriptor::interrupt(_exc10 as u32),
        IDTDescriptor::interrupt(_exc11 as u32),
        IDTDescriptor::interrupt(_exc12 as u32),
        IDTDescriptor::interrupt(_exc13 as u32),
        IDTDescriptor::interrupt(_exc14 as u32),
        IDTDescriptor::interrupt(_exc15 as u32),
        IDTDescriptor::interrupt(_exc16 as u32),
        IDTDescriptor::interrupt(_exc17 as u32),
        IDTDescriptor::interrupt(_exc18 as u32),
        IDTDescriptor::interrupt(_exc19 as u32),
        IDTDescriptor::interrupt(_exc20 as u32),
        IDTDescriptor::interrupt(_exc21 as u32),
        IDTDescriptor::interrupt(_exc22 as u32),
        IDTDescriptor::interrupt(_exc23 as u32),
        IDTDescriptor::interrupt(_exc24 as u32),
        IDTDescriptor::interrupt(_exc25 as u32),
        IDTDescriptor::interrupt(_exc26 as u32),
        IDTDescriptor::interrupt(_exc27 as u32),
        IDTDescriptor::interrupt(_exc28 as u32),
        IDTDescriptor::interrupt(_exc29 as u32),
        IDTDescriptor::interrupt(_exc30 as u32),
        IDTDescriptor::interrupt(_exc31 as u32),

        // 0x20..0x30 - PIC interrupts
        // IDTDescriptor::interrupt(_irq0 as u32),
        // IDTDescriptor::interrupt(_irq1 as u32),
        // IDTDescriptor::interrupt(_irq2 as u32),
        // IDTDescriptor::interrupt(_irq3 as u32),
        // IDTDescriptor::interrupt(_irq4 as u32),
        // IDTDescriptor::interrupt(_irq5 as u32),
        // IDTDescriptor::interrupt(_irq6 as u32),
        // IDTDescriptor::interrupt(_irq7 as u32),
        // IDTDescriptor::interrupt(_irq8 as u32),
        // IDTDescriptor::interrupt(_irq9 as u32),
        // IDTDescriptor::interrupt(_irq10 as u32),
        // IDTDescriptor::interrupt(_irq11 as u32),
        // IDTDescriptor::interrupt(_irq12 as u32),
        // IDTDescriptor::interrupt(_irq13 as u32),
        // IDTDescriptor::interrupt(_irq14 as u32),
        // IDTDescriptor::interrupt(_irq15 as u32),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),

        // 0x30..0xff - ??
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
        IDTDescriptor::interrupt(0),
    ];
}

static mut IDTA: IDTArray = IDTArray {
    limit: 0,
    base: 0,
};

pub fn load_idt() {
    unsafe {
        IDTA.limit = 256 * core::mem::size_of::<IDTDescriptor>() as u16 - 1;
        IDTA.base = IDT.as_ptr() as u32;
        _load_idt(&IDTA as *const IDTArray);
    }
}

impl IDTDescriptor {
    fn interrupt(offset: u32) -> IDTDescriptor {
        let mut type_attr = 0x0e; // 32-bit interrupt
        if offset != 0 {
            type_attr |= 0x80;
        }
        IDTDescriptor {
            offset0: offset as u16,
            selector: 0x08, // kernel code selector (offset), see gdt.rs
            zero: 0,
            type_attr: type_attr,
            offset1: (offset >> 16) as u16,
        }
    }
}

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
    println!("[IRQ] Got IRQ0");
    io::outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq1_handler() {
    println!("[IRQ] Got IRQ1");
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
