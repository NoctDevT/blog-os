;entry point to the kernel
;default section for .text for executable code
;startr 32 bit because our cpu will be in protected mode
;when grub starts our kernel in long mode we can switch to
;64 bit mode
; mov dword moves 32 bit constraibt to address b8000
global start

section .text
bits 32
start:
    ; print `OK` to screen
    mov dword [0xb800], 0x2f4b2f4f
    hlt
