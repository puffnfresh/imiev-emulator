    .section .text.start,"ax"
    .global _start
_start:
    ld24  r15,#0x00813000
    bl    main
    ld24  r1,#0x00800000
    st    r0,@r1
.Lspin:
    bra   .Lspin
