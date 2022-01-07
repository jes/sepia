    .global _start

_start:

    # only run on hart id 0
    csrr a0, mhartid
    bne a0, x0, finish

    lui t0, 0x10010
    la t1, string

loop:
    lb t2, 0(t1)
    beq t2, x0, finish
    sw t2, 0(t0)
    addi t1, t1, 1
    j loop

finish:
    j finish

string: .asciz "Hello, world!\n";
