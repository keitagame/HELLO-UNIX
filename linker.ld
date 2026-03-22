/* linker.ld - Kernel linker script */

ENTRY(_start)

SECTIONS
{
    /* Load at 1 MiB - standard for Multiboot kernels */
    . = 1M;

    /* Multiboot header must be in the first 32KB of the kernel image */
    .multiboot2 :
    {
        KEEP(*(.multiboot2))
    }

    /* Code section */
    .text ALIGN(4K) :
    {
        *(.text*)
    }

    /* Read-only data */
    .rodata ALIGN(4K) :
    {
        *(.rodata*)
    }

    /* Read-write data (initialized) */
    .data ALIGN(4K) :
    {
        *(.data*)
    }

    /* BSS (zero-initialized) */
    .bss ALIGN(4K) :
    {
        *(COMMON)
        *(.bss*)
    }
}
