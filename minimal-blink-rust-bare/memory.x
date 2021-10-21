MEMORY {
    BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x100
    FLASH : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100
    RAM   : ORIGIN = 0x20000000, LENGTH = 256K
}

/* The entry point is the reset handler */
ENTRY(Reset);

SECTIONS {
    /* ### Boot loader */
    .boot2 ORIGIN(BOOT2) :
    {
        KEEP(*(.boot2));
    } > BOOT2

    .vector_table ORIGIN(FLASH) :
    {
       /* First entry: initial Stack Pointer value */
       LONG(ORIGIN(RAM) + LENGTH(RAM));

       /* Second entry: reset vector */
       KEEP(*(.vector_table.reset_vector));

       /* Remaining entries: other exceptions */
       KEEP(*(.vector_table.exceptions));
    } > FLASH

    .text :
    {
       *(.text .text.*);
    } > FLASH

    .rodata :
    {
       *(.rodata .rodata.*);
    } > FLASH

    /DISCARD/ :
    {
       *(.ARM.exidx .ARM.exidx.*);
    }
}
