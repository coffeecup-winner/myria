OUTPUT_FORMAT(elf32-i386)
ENTRY(entry)
SECTIONS
{
    . = 1M;
    .boot : { *(.boot) }
    .text : { *(.text) }
}
