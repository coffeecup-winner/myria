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
section .text
bits 32
entry:
    cli
    mov dword [0xb8000], 0x07690748
    hlt
