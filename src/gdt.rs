#[repr(C)]
#[repr(packed)]
struct GDTEntry {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

static GDT: [GDTEntry; 3] = [
    // Null descriptor
    GDTEntry {
        limit_low: 0x0000,
        base_low: 0x0000,
        base_mid: 0x00,
        access: 0x00,
        granularity: 0x00,
        base_high: 0x00,
    },
    // Code descriptor (priv)
    GDTEntry {
        limit_low: 0xffff,
        base_low: 0x0000,
        base_mid: 0x00,
        access: 0x9a,
        granularity: 0xcf,
        base_high: 0x00,
    },
    // Data descriptor (priv)
    GDTEntry {
        limit_low: 0xffff,
        base_low: 0x0000,
        base_mid: 0x00,
        access: 0x92,
        granularity: 0xcf,
        base_high: 0x00,
    },
];

#[repr(C)]
#[repr(packed)]
struct GDTPointer
{
    limit: u16,
    base: u32,
}

static mut GDTP: GDTPointer = GDTPointer {
    limit: 0,
    base: 0,
};

extern "C" {
    fn _load_gdt(gdtp: *const GDTPointer);
}

pub fn load_gdt() {
    unsafe {
        GDTP.limit = (3 * core::mem::size_of::<GDTEntry>()) as u16;
        GDTP.base = GDT.as_ptr() as u32;
        _load_gdt(&GDTP as *const GDTPointer);
    }
}
