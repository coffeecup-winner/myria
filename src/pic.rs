use crate::io;

pub const PIC0_CMD: u16 = 0x20;
pub const PIC0_DATA: u16 = 0x21;
pub const PIC1_CMD: u16 = 0xa0;
pub const PIC1_DATA: u16 = 0xa1;

pub const PIC_CMD_EOI: u8 = 0x20;

const PIC_CMD_ICW1_ICW4: u8 = 0x01; // whether ICW4 is needed
const PIC_CMD_ICW1_SINGLE: u8 = 0x02; // single/cascade mode
const PIC_CMD_ICW1_INTERVAL4: u8 = 0x04; // interval 4/8
const PIC_CMD_ICW1_LEVEL: u8 = 0x08; // level/edge triggered
const PIC_CMD_ICW1_INIT: u8 = 0x10; // initialization

// ICW2 and ICW3 are inline below

const PIC_CMD_ICW4_8086: u8 = 0x01; // 8086/88 mode
const PIC_CMD_ICW4_AUTO: u8 = 0x02; // auto/normal EOI
const PIC_CMD_ICW4_BUF_SLAVE: u8 = 0x04; // buffered mode - slave
const PIC_CMD_ICW4_BUF_MASTER: u8 = 0x08; // buffered mode - master
const PIC_CMD_ICW4_SFNM: u8 = 0x10; // special fully nested

pub fn remap(offset0: u8, offset1: u8) {
    unsafe {
        let pic0mask = io::_inb(PIC0_DATA);
        let pic1mask = io::_inb(PIC1_DATA);

        // ICW1
        io::_outb(PIC0_CMD, PIC_CMD_ICW1_INIT | PIC_CMD_ICW1_ICW4);
        io::_iowait();
        io::_outb(PIC1_CMD, PIC_CMD_ICW1_INIT | PIC_CMD_ICW1_ICW4);
        io::_iowait();

        // ICW2
        io::_outb(PIC0_DATA, offset0);
        io::_iowait();
        io::_outb(PIC1_DATA, offset1);
        io::_iowait();

        // ICW3
        io::_outb(PIC0_DATA, 1 << 2); // slave PIC at IRQ 2 in master
        io::_iowait();
        io::_outb(PIC1_DATA, 1 << 1); // slave PIC's cascade identity
        io::_iowait();

        // ICW4
        io::_outb(PIC0_DATA, PIC_CMD_ICW4_8086);
        io::_iowait();
        io::_outb(PIC1_DATA, PIC_CMD_ICW4_8086);
        io::_iowait();

        io::_outb(PIC0_DATA, pic0mask);
        io::_outb(PIC1_DATA, pic1mask);
    }
}
