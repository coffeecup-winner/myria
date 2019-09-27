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
global _outb
global _inb
global _iowait
global _exc0
global _exc1
global _exc2
global _exc3
global _exc4
global _exc5
global _exc6
global _exc7
global _exc8
global _exc9
global _exc10
global _exc11
global _exc12
global _exc13
global _exc14
global _exc15
global _exc16
global _exc17
global _exc18
global _exc19
global _exc20
global _exc21
global _exc22
global _exc23
global _exc24
global _exc25
global _exc26
global _exc27
global _exc28
global _exc29
global _exc30
global _exc31
global _irq0
global _irq1
global _irq2
global _irq3
global _irq4
global _irq5
global _irq6
global _irq7
global _irq8
global _irq9
global _irq10
global _irq11
global _irq12
global _irq13
global _irq14
global _irq15

extern _entry
extern error_code
extern exc0_handler
extern exc1_handler
extern exc2_handler
extern exc3_handler
extern exc4_handler
extern exc5_handler
extern exc6_handler
extern exc7_handler
extern exc8_handler
extern exc9_handler
extern exc10_handler
extern exc11_handler
extern exc12_handler
extern exc13_handler
extern exc14_handler
extern exc15_handler
extern exc16_handler
extern exc17_handler
extern exc18_handler
extern exc19_handler
extern exc20_handler
extern exc21_handler
extern exc22_handler
extern exc23_handler
extern exc24_handler
extern exc25_handler
extern exc26_handler
extern exc27_handler
extern exc28_handler
extern exc29_handler
extern exc30_handler
extern exc31_handler
extern irq0_handler
extern irq1_handler
extern irq2_handler
extern irq3_handler
extern irq4_handler
extern irq5_handler
extern irq6_handler
extern irq7_handler
extern irq8_handler
extern irq9_handler
extern irq10_handler
extern irq11_handler
extern irq12_handler
extern irq13_handler
extern irq14_handler
extern irq15_handler

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
    sti
    ret

_outb: ; _outb(u16, u8)
    mov dx, [esp + 4]
    mov al, [esp + 6]
    out dx, al
    ret

_inb: ; _inb(u16) -> u8
    mov dx, [esp + 4]
    in al, dx
    ret

_iowait: ; _iowait()
    xor eax, eax
    out 0x80, al ; waste a cycle on an unused port
    ret

_exc0: ; _exc0()
    pusha
    call exc0_handler
    popa
    iret

_exc1: ; _exc1()
    pusha
    call exc1_handler
    popa
    iret

_exc2: ; _exc2()
    pusha
    call exc2_handler
    popa
    iret

_exc3: ; _exc3()
    pusha
    call exc3_handler
    popa
    iret

_exc4: ; _exc4()
    pusha
    call exc4_handler
    popa
    iret

_exc5: ; _exc5()
    pusha
    call exc5_handler
    popa
    iret

_exc6: ; _exc6()
    pusha
    call exc6_handler
    popa
    iret

_exc7: ; _exc7()
    pusha
    call exc7_handler
    popa
    iret

_exc8: ; _exc8()
    push eax
    mov eax, [esp + 4] ; error code
    pusha
    push eax
    call exc8_handler
    popa
    pop eax
    add esp, 4 ; remove error code
    iret

_exc9: ; _exc9()
    pusha
    call exc9_handler
    popa
    iret

_exc10: ; _exc10()
    push eax
    mov eax, [esp + 4] ; error code
    pusha
    push eax
    call exc10_handler
    popa
    pop eax
    add esp, 4 ; remove error code
    iret

_exc11: ; _exc11()
    push eax
    mov eax, [esp + 4] ; error code
    pusha
    push eax
    call exc11_handler
    popa
    pop eax
    add esp, 4 ; remove error code
    iret

_exc12: ; _exc12()
    push eax
    mov eax, [esp + 4] ; error code
    pusha
    push eax
    call exc12_handler
    popa
    pop eax
    add esp, 4 ; remove error code
    iret

_exc13: ; _exc13()
    push eax
    mov eax, [esp + 4] ; error code
    pusha
    push eax
    call exc13_handler
    popa
    pop eax
    add esp, 4 ; remove error code
    iret

_exc14: ; _exc14()
    push eax
    mov eax, [esp + 4] ; error code
    pusha
    push eax
    call exc14_handler
    popa
    pop eax
    add esp, 4 ; remove error code
    iret

_exc15: ; _exc15()
    pusha
    call exc15_handler
    popa
    iret

_exc16: ; _exc16()
    pusha
    call exc16_handler
    popa
    iret

_exc17: ; _exc17()
    push eax
    mov eax, [esp + 4] ; error code
    pusha
    push eax
    call exc17_handler
    popa
    pop eax
    add esp, 4 ; remove error code
    iret

_exc18: ; _exc18()
    pusha
    call exc18_handler
    popa
    iret

_exc19: ; _exc19()
    pusha
    call exc19_handler
    popa
    iret

_exc20: ; _exc20()
    pusha
    call exc20_handler
    popa
    iret

_exc21: ; _exc21()
    pusha
    call exc21_handler
    popa
    iret

_exc22: ; _exc22()
    pusha
    call exc22_handler
    popa
    iret

_exc23: ; _exc23()
    pusha
    call exc23_handler
    popa
    iret

_exc24: ; _exc24()
    pusha
    call exc24_handler
    popa
    iret

_exc25: ; _exc25()
    pusha
    call exc25_handler
    popa
    iret

_exc26: ; _exc26()
    pusha
    call exc26_handler
    popa
    iret

_exc27: ; _exc27()
    pusha
    call exc27_handler
    popa
    iret

_exc28: ; _exc28()
    pusha
    call exc28_handler
    popa
    iret

_exc29: ; _exc29()
    pusha
    call exc29_handler
    popa
    iret

_exc30: ; _exc30()
    push eax
    mov eax, [esp + 4] ; error code
    pusha
    push eax
    call exc30_handler
    popa
    pop eax
    add esp, 4 ; remove error code
    iret

_exc31: ; _exc31()
    pusha
    call exc31_handler
    popa
    iret

_irq0: ; _irq0()
    pusha
    call irq0_handler
    popa
    iret

_irq1: ; _irq1()
    pusha
    call irq1_handler
    popa
    iret

_irq2: ; _irq2()
    pusha
    call irq2_handler
    popa
    iret

_irq3: ; _irq3()
    pusha
    call irq3_handler
    popa
    iret

_irq4: ; _irq4()
    pusha
    call irq4_handler
    popa
    iret

_irq5: ; _irq5()
    pusha
    call irq5_handler
    popa
    iret

_irq6: ; _irq6()
    pusha
    call irq6_handler
    popa
    iret

_irq7: ; _irq7()
    pusha
    call irq7_handler
    popa
    iret

_irq8: ; _irq8()
    pusha
    call irq8_handler
    popa
    iret

_irq9: ; _irq9()
    pusha
    call irq9_handler
    popa
    iret

_irq10: ; _irq10()
    pusha
    call irq10_handler
    popa
    iret

_irq11: ; _irq11()
    pusha
    call irq11_handler
    popa
    iret

_irq12: ; _irq12()
    pusha
    call irq12_handler
    popa
    iret

_irq13: ; _irq13()
    pusha
    call irq13_handler
    popa
    iret

_irq14: ; _irq14()
    pusha
    call irq14_handler
    popa
    iret

_irq15: ; _irq15()
    pusha
    call irq15_handler
    popa
    iret
