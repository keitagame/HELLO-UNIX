; boot.asm - Multiboot2 header + 32→64bit long mode setup

BITS 32

; Multiboot2 constants
MULTIBOOT2_MAGIC    equ 0xE85250D6
MULTIBOOT2_ARCH     equ 0
HEADER_LENGTH       equ (multiboot2_header_end - multiboot2_header_start)
CHECKSUM            equ (0x100000000 - (MULTIBOOT2_MAGIC + MULTIBOOT2_ARCH + HEADER_LENGTH))

section .multiboot2
align 8
multiboot2_header_start:
    dd MULTIBOOT2_MAGIC
    dd MULTIBOOT2_ARCH
    dd HEADER_LENGTH
    dd CHECKSUM
    ; End tag
    dw 0
    dw 0
    dd 8
multiboot2_header_end:


section .bss
align 8
saved_magic:   resd 1
saved_mbi:     resd 1
align 16
stack_bottom:
    resb 16384
stack_top:

; Page tables for identity mapping (4MB)
align 4096
pml4_table:
    resb 4096
pdp_table:
    resb 4096
pd_table:
    resb 4096

section .rodata
gdt64:
    dq 0
gdt64_code:
    dq (1<<43) | (1<<44) | (1<<47) | (1<<53)

gdt64_desc:
    dw gdt64_desc_end - gdt64 - 1
    dq gdt64
gdt64_desc_end:

.code: equ $ - gdt64
    dq (1<<43) | (1<<44) | (1<<47) | (1<<53)  ; code segment
.pointer:
    dw $ - gdt64 - 1
    dq gdt64

section .text
global _start
extern kernel_main

_start:
    mov esp, stack_top
    push 0
    popf
    mov [saved_magic], eax
    mov [saved_mbi],   ebx
    ; --- Check multiboot2 ---
    cmp eax, 0x36D76289
    jne .error

    ; --- Set up paging for long mode ---
    ; PML4[0] -> PDP
   ; PML4[0] = PDP
mov eax, pdp_table
or eax, 0x3
mov dword [pml4_table], eax
mov dword [pml4_table+4], 0

; PDP[0] = PD
mov eax, pd_table
or eax, 0x3
mov dword [pdp_table], eax
mov dword [pdp_table+4], 0

; PD entries (2MiB pages)
mov ecx, 0
.map_pd:
    mov eax, 0x200000
    mul ecx
    or eax, 0x83
    mov dword [pd_table + ecx*8], eax
    mov dword [pd_table + ecx*8 + 4], 0
    inc ecx
    cmp ecx, 4
    jne .map_pd



    ; Load PML4
    mov eax, pml4_table
    mov cr3, eax

    ; Enable PAE
    mov eax, cr4
    or eax, (1 << 5)
    mov cr4, eax

    ; Enable Long Mode (EFER.LME)
    mov ecx, 0xC0000080
    rdmsr
    or eax, (1 << 8)
    wrmsr

    ; Enable paging + protected mode
    mov eax, cr0
    or eax, (1 << 31) | (1 << 0)
    mov cr0, eax

    ; Load 64-bit GDT
    ;lgdt [gdt64.pointer]

    ; Far jump to 64-bit code
    ;jmp gdt64.code:long_mode_start
lgdt [gdt64_desc]
jmp 0x08:long_mode_start

.error:
    ; Write 'ERR' to VGA
    mov word [0xb8000], 0x4f45   ; 'E' red
    mov word [0xb8002], 0x4f52   ; 'R'
    mov word [0xb8004], 0x4f52   ; 'R'
.hang:
    hlt
    jmp .hang

BITS 64
long_mode_start:
    ; Zero segment registers
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov eax, dword [saved_magic]
    mov ebx, dword [saved_mbi]
    mov edi, eax        ; 第1引数 (u32 magic)
    mov esi, ebx        ; 第2引数 (u32 mbi_phys)
    call kernel_main

.hang:
    cli
    hlt
    jmp .hang
