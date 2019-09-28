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
