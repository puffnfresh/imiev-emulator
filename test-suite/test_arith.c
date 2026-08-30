/*
 * Self-checking M32R ISA smoke test for the emulator's instruction interpreter.
 * Returns 0 on success, or the id of the first failing check (so a red test tells
 * you exactly which instruction class is wrong). Uses only native M32R ops — no
 * division or 64-bit math (those would pull in libgcc, which we don't link).
 */
#define CHECK(id, cond) do { if (!(cond)) return (id); } while (0)

int main(void)
{
    volatile int a = 7, b = 5;          /* volatile => real loads/stores, not folded */

    CHECK(1,  a + b == 12);             /* add  */
    CHECK(2,  a - b == 2);              /* sub  */
    CHECK(3,  a * b == 35);             /* mul  */
    CHECK(4,  (a << 2) == 28);          /* sll  */
    CHECK(5,  (a >> 1) == 3);           /* sra  */
    CHECK(6,  (a | b) == 7);            /* or   */
    CHECK(7,  (a & b) == 5);            /* and  */
    CHECK(8,  (a ^ b) == 2);            /* xor  */

    volatile unsigned u = 0x80000000u;
    CHECK(9,  (u >> 1) == 0x40000000u); /* srl (logical) */
    CHECK(10, ((int)u >> 1) == (int)0xC0000000u); /* sra (arithmetic, sign fill) */

    /* signed compare / branch */
    CHECK(11, (a > b) && !(b > a));
    CHECK(12, ((int)0xFFFFFFF6 + 20) == 10); /* -10 + 20, carry across sign */

    /* memory round-trip through the stack */
    volatile int arr[3];
    arr[0] = a; arr[1] = b; arr[2] = a + b;
    CHECK(13, arr[2] == 12 && arr[0] == 7 && arr[1] == 5);

    return 0;
}
