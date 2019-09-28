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
extern exception_handler
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

_exc0: ; _exc0()
    push dword 0
    push dword 0
    jmp _exc_common

_exc1: ; _exc1()
    push dword 0
    push dword 1
    jmp _exc_common

_exc2: ; _exc2()
    push dword 0
    push dword 2
    jmp _exc_common

_exc3: ; _exc3()
    push dword 0
    push dword 3
    jmp _exc_common

_exc4: ; _exc4()
    push dword 0
    push dword 4
    jmp _exc_common

_exc5: ; _exc5()
    push dword 0
    push dword 5
    jmp _exc_common

_exc6: ; _exc6()
    push dword 0
    push dword 6
    jmp _exc_common

_exc7: ; _exc7()
    push dword 0
    push dword 7
    jmp _exc_common

_exc8: ; _exc8()
    push dword 8
    jmp _exc_common

_exc9: ; _exc9()
    push dword 0
    push dword 9
    jmp _exc_common

_exc10: ; _exc10()
    push dword 10
    jmp _exc_common

_exc11: ; _exc11()
    push dword 11
    jmp _exc_common

_exc12: ; _exc12()
    push dword 12
    jmp _exc_common

_exc13: ; _exc13()
    push dword 13
    jmp _exc_common

_exc14: ; _exc14()
    push dword 14
    jmp _exc_common

_exc15: ; _exc15()
    push dword 0
    push dword 15
    jmp _exc_common

_exc16: ; _exc16()
    push dword 0
    push dword 16
    jmp _exc_common

_exc17: ; _exc17()
    push dword 17
    jmp _exc_common

_exc18: ; _exc18()
    push dword 0
    push dword 18
    jmp _exc_common

_exc19: ; _exc19()
    push dword 0
    push dword 19
    jmp _exc_common

_exc20: ; _exc20()
    push dword 0
    push dword 20
    jmp _exc_common

_exc21: ; _exc21()
    push dword 0
    push dword 21
    jmp _exc_common

_exc22: ; _exc22()
    push dword 0
    push dword 22
    jmp _exc_common

_exc23: ; _exc23()
    push dword 0
    push dword 23
    jmp _exc_common

_exc24: ; _exc24()
    push dword 0
    push dword 24
    jmp _exc_common

_exc25: ; _exc25()
    push dword 0
    push dword 25
    jmp _exc_common

_exc26: ; _exc26()
    push dword 0
    push dword 26
    jmp _exc_common

_exc27: ; _exc27()
    push dword 0
    push dword 27
    jmp _exc_common

_exc28: ; _exc28()
    push dword 0
    push dword 28
    jmp _exc_common

_exc29: ; _exc29()
    push dword 0
    push dword 29
    jmp _exc_common

_exc30: ; _exc30()
    push dword 30
    jmp _exc_common

_exc31: ; _exc31()
    push dword 0
    push dword 31
    jmp _exc_common

_exc_common:
    pushad
    push ds
    push es
    push fs
    push gs
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov eax, esp
    push eax
    mov eax, exception_handler
    call eax
    pop eax
    pop gs
    pop fs
    pop es
    pop ds
    popad
    add esp, 8
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
