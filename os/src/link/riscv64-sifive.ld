OUTPUT_ARCH("riscv")

ENTRY(_start)

MEMORY
{
    /* HiFive Pro Memory Map */
    FLASH (rx) : ORIGIN = 0x20000000, LENGTH = 128M
    RAM (rwx)  : ORIGIN = 0x80000000, LENGTH = 8M
}

SECTIONS
{
    /* Boot section */
    .text.init : {
        *(.init.rust)
        *(.trap_vector)
        *(.init)
    } > FLASH

    /* Code section */
    .text : {
        *(.text.*)
        *(.text)
    } > FLASH

    /* Read-only data */
    .rodata : {
        *(.rodata.*)
        *(.rodata)
    } > FLASH

    /* Data section */
    .data : {
        . = ALIGN(4);
        *(.data.*)
        *(.data)
        . = ALIGN(4);
    } > RAM AT > FLASH

    /* BSS section */
    .bss : {
        . = ALIGN(4);
        __bss_start = .;
        *(.bss.*)
        *(.bss)
        . = ALIGN(4);
        __bss_end = .;
    } > RAM

    /* Stack section */
    .stack : {
        . = ALIGN(16);
        __stack_bottom = .;
        . += 4K;
        __stack_top = .;
    } > RAM
}