/*
 * Carry / borrow propagation. 64-bit add/sub inline to add+addx / sub+subx on M32R
 * (verified: no libgcc call), so these directly exercise the emulator's ADDX/SUBX and
 * the C flag. Also unsigned overflow detection and signed-vs-unsigned compare.
 * Returns 0 on success, else the id of the first failing check.
 */
#define CHECK(id, cond) do { if (!(cond)) return (id); } while (0)

int main(void)
{
    /* carry out of the low word into the high word (add + addx) */
    volatile unsigned long long p = 0x00000000FFFFFFFFull;
    volatile unsigned long long q = 0x0000000000000001ull;
    CHECK(1, (p + q) == 0x0000000100000000ull);

    /* full 64-bit wrap */
    volatile unsigned long long r = 0xFFFFFFFFFFFFFFFFull;
    CHECK(2, (r + 1ull) == 0ull);

    /* borrow from the high word (sub + subx) */
    volatile unsigned long long s = 0x0000000100000000ull;
    CHECK(3, (s - 1ull) == 0x00000000FFFFFFFFull);

    /* mixed: add then subtract back is identity across the word boundary */
    volatile unsigned long long t = 0x00000000FFFFFFFFull;
    CHECK(4, ((t + 0x0000000000000002ull) - 0x0000000000000002ull) == t);

    /* unsigned 32-bit overflow: wrapped sum is smaller than an addend */
    volatile unsigned x = 0xFFFFFFF0u, y = 0x20u;
    unsigned sum = x + y;
    CHECK(5, sum == 0x10u && sum < x);

    /* increment across a byte-carry boundary */
    volatile unsigned a = 0x00FFFFFFu;
    CHECK(6, (a + 1u) == 0x01000000u);

    /* signed compare (cmp) vs unsigned compare (cmpu) diverge on the high bit */
    volatile int si = -1;   /* 0xFFFFFFFF */
    volatile int sj = 1;
    CHECK(7, si < sj);                            /* signed:   -1 < 1        */
    CHECK(8, (unsigned)si > (unsigned)sj);        /* unsigned: 0xFFFFFFFF > 1 */

    return 0;
}
