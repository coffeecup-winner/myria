use crate::io;
use crate::pic;

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
        IDTDescriptor::interrupt(_irq0 as u32),
        IDTDescriptor::interrupt(_irq1 as u32),
        IDTDescriptor::interrupt(_irq2 as u32),
        IDTDescriptor::interrupt(_irq3 as u32),
        IDTDescriptor::interrupt(_irq4 as u32),
        IDTDescriptor::interrupt(_irq5 as u32),
        IDTDescriptor::interrupt(_irq6 as u32),
        IDTDescriptor::interrupt(_irq7 as u32),
        IDTDescriptor::interrupt(_irq8 as u32),
        IDTDescriptor::interrupt(_irq9 as u32),
        IDTDescriptor::interrupt(_irq10 as u32),
        IDTDescriptor::interrupt(_irq11 as u32),
        IDTDescriptor::interrupt(_irq12 as u32),
        IDTDescriptor::interrupt(_irq13 as u32),
        IDTDescriptor::interrupt(_irq14 as u32),
        IDTDescriptor::interrupt(_irq15 as u32),

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

#[no_mangle]
pub unsafe extern "C" fn exc0_handler() {
    println!("[EXC] Got exception 0");
}

#[no_mangle]
pub unsafe extern "C" fn exc1_handler() {
    println!("[EXC] Got exception 1");
}

#[no_mangle]
pub unsafe extern "C" fn exc2_handler() {
    println!("[EXC] Got exception 2");
}

#[no_mangle]
pub unsafe extern "C" fn exc3_handler() {
    println!("[EXC] Got exception 3");
}

#[no_mangle]
pub unsafe extern "C" fn exc4_handler() {
    println!("[EXC] Got exception 4");
}

#[no_mangle]
pub unsafe extern "C" fn exc5_handler() {
    println!("[EXC] Got exception 5");
}

#[no_mangle]
pub unsafe extern "C" fn exc6_handler() {
    println!("[EXC] Got exception 6");
}

#[no_mangle]
pub unsafe extern "C" fn exc7_handler() {
    println!("[EXC] Got exception 7");
}

#[no_mangle]
pub unsafe extern "C" fn exc8_handler(error_code: u32) {
    println!("[EXC] Got exception 8: 0x{:x}", error_code);
}

#[no_mangle]
pub unsafe extern "C" fn exc9_handler() {
    println!("[EXC] Got exception 9");
}

#[no_mangle]
pub unsafe extern "C" fn exc10_handler(error_code: u32) {
    println!("[EXC] Got exception 10: 0x{:x}", error_code);
}

#[no_mangle]
pub unsafe extern "C" fn exc11_handler(error_code: u32) {
    println!("[EXC] Got exception 11: 0x{:x}", error_code);
}

#[no_mangle]
pub unsafe extern "C" fn exc12_handler(error_code: u32) {
    println!("[EXC] Got exception 12: 0x{:x}", error_code);
}

#[no_mangle]
pub unsafe extern "C" fn exc13_handler(error_code: u32) {
    println!("[EXC] Got exception 13: 0x{:x}", error_code);
}

#[no_mangle]
pub unsafe extern "C" fn exc14_handler(error_code: u32) {
    println!("[EXC] Got exception 14: 0x{:x}", error_code);
}

#[no_mangle]
pub unsafe extern "C" fn exc15_handler() {
    println!("[EXC] Got exception 15");
}

#[no_mangle]
pub unsafe extern "C" fn exc16_handler() {
    println!("[EXC] Got exception 16");
}

#[no_mangle]
pub unsafe extern "C" fn exc17_handler(error_code: u32) {
    println!("[EXC] Got exception 17: 0x{:x}", error_code);
}

#[no_mangle]
pub unsafe extern "C" fn exc18_handler() {
    println!("[EXC] Got exception 18");
}

#[no_mangle]
pub unsafe extern "C" fn exc19_handler() {
    println!("[EXC] Got exception 19");
}

#[no_mangle]
pub unsafe extern "C" fn exc20_handler() {
    println!("[EXC] Got exception 20");
}

#[no_mangle]
pub unsafe extern "C" fn exc21_handler() {
    println!("[EXC] Got exception 21");
}

#[no_mangle]
pub unsafe extern "C" fn exc22_handler() {
    println!("[EXC] Got exception 22");
}

#[no_mangle]
pub unsafe extern "C" fn exc23_handler() {
    println!("[EXC] Got exception 23");
}

#[no_mangle]
pub unsafe extern "C" fn exc24_handler() {
    println!("[EXC] Got exception 24");
}

#[no_mangle]
pub unsafe extern "C" fn exc25_handler() {
    println!("[EXC] Got exception 25");
}

#[no_mangle]
pub unsafe extern "C" fn exc26_handler() {
    println!("[EXC] Got exception 26");
}

#[no_mangle]
pub unsafe extern "C" fn exc27_handler() {
    println!("[EXC] Got exception 27");
}

#[no_mangle]
pub unsafe extern "C" fn exc28_handler() {
    println!("[EXC] Got exception 28");
}

#[no_mangle]
pub unsafe extern "C" fn exc29_handler() {
    println!("[EXC] Got exception 29");
}

#[no_mangle]
pub unsafe extern "C" fn exc30_handler(error_code: u32) {
    println!("[EXC] Got exception 30: 0x{:x}", error_code);
}

#[no_mangle]
pub unsafe extern "C" fn exc31_handler() {
    println!("[EXC] Got exception 31");
}

#[no_mangle]
pub unsafe extern "C" fn irq0_handler() {
    println!("[IRQ] Got IRQ0");
    io::_outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq1_handler() {
    println!("[IRQ] Got IRQ1");
    io::_outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq2_handler() {
    println!("[IRQ] Got IRQ2");
    io::_outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq3_handler() {
    println!("[IRQ] Got IRQ3");
    io::_outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq4_handler() {
    println!("[IRQ] Got IRQ4");
    io::_outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq5_handler() {
    println!("[IRQ] Got IRQ5");
    io::_outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq6_handler() {
    println!("[IRQ] Got IRQ6");
    io::_outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq7_handler() {
    println!("[IRQ] Got IRQ7");
    io::_outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq8_handler() {
    println!("[IRQ] Got IRQ8");
    io::_outb(pic::PIC1_CMD, pic::PIC_CMD_EOI);
    io::_outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq9_handler() {
    println!("[IRQ] Got IRQ9");
    io::_outb(pic::PIC1_CMD, pic::PIC_CMD_EOI);
    io::_outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq10_handler() {
    println!("[IRQ] Got IRQ10");
    io::_outb(pic::PIC1_CMD, pic::PIC_CMD_EOI);
    io::_outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq11_handler() {
    println!("[IRQ] Got IRQ11");
    io::_outb(pic::PIC1_CMD, pic::PIC_CMD_EOI);
    io::_outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq12_handler() {
    println!("[IRQ] Got IRQ12");
    io::_outb(pic::PIC1_CMD, pic::PIC_CMD_EOI);
    io::_outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq13_handler() {
    println!("[IRQ] Got IRQ13");
    io::_outb(pic::PIC1_CMD, pic::PIC_CMD_EOI);
    io::_outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq14_handler() {
    println!("[IRQ] Got IRQ14");
    io::_outb(pic::PIC1_CMD, pic::PIC_CMD_EOI);
    io::_outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}

#[no_mangle]
pub unsafe extern "C" fn irq15_handler() {
    println!("[IRQ] Got IRQ15");
    io::_outb(pic::PIC1_CMD, pic::PIC_CMD_EOI);
    io::_outb(pic::PIC0_CMD, pic::PIC_CMD_EOI);
}
