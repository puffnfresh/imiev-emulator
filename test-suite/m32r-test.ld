/* Linker script for bare-metal M32R emulator C tests.
 * Code + rodata live in flash from 0x0 (executable image). RAM holds .bss and
 * the stack. NOTE: 0x800000 is reserved as the RESULT slot (see crt0.S), so RAM
 * output sections start above it. Initialized, MUTABLE globals are not supported
 * yet (no .data copy in crt0) — keep tests to locals/rodata for now. */
OUTPUT_FORMAT("elf32-m32r")
OUTPUT_ARCH(m32r)
ENTRY(_start)

MEMORY {
  FLASH (rx)  : ORIGIN = 0x00000000, LENGTH = 1024K
  RAM   (rwx) : ORIGIN = 0x00801000, LENGTH = 0x00012000  /* 0x801000..0x813000 */
}

SECTIONS {
  .text 0x0 : {
    KEEP(*(.text.start))
    *(.text*)
    *(.rodata*)
    *(.data*)          /* folded into flash: OK for read-only initializers */
  } > FLASH

  .bss (NOLOAD) : {
    __bss_start = .;
    *(.bss*)
    *(COMMON)
    __bss_end = .;
  } > RAM

  /DISCARD/ : { *(.comment) *(.note*) *(.eh_frame*) }
}
