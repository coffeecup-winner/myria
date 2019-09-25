section .boot
boot_start:
    dd 0xe85250d6
    dd 0x0
    dd boot_end - boot_start
    dd 0x100000000 - (0xe85250d6 + 0x0 + (boot_end - boot_start))
    dw 0x0
    dw 0x0
    dd 0x8
boot_end:

global entry
global _load_gdt
global _load_idt

extern _entry

section .text
bits 32
entry:
    cli
    jmp _entry
    hlt

_load_gdt: ; _load_gdt(u32)
    mov eax, [esp + 4]
    lgdt [eax]
    jmp 0x08:flush ; flushes cs
flush:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    ret

_load_idt: ; _load_idt(u32)
    mov eax, [esp + 4]
    lidt [eax]
    ret
