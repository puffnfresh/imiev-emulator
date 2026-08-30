/*
 * 32-bit multiply and shifts. All results stay in 32 bits (no widening/64-bit mul,
 * which would call libgcc __muldi3). Multiplies by constants let gcc strength-reduce
 * to shift/add sequences — we verify the numeric result either way.
 * Returns 0 on success, else the id of the first failing check.
 */
#define CHECK(id, cond) do { if (!(cond)) return (id); } while (0)

int main(void)
{
    volatile int a = 1000, b = 1000;
    CHECK(1, a * b == 1000000);          /* mul, fits in 32 bits */

    volatile int x = 65536, y = 65536;
    CHECK(2, x * y == 0);                /* 2^32 wraps to 0 in the low word */

    volatile unsigned u = 0xFFFFFFFFu;
    CHECK(3, u * 2u == 0xFFFFFFFEu);     /* unsigned wrap */

    volatile int n = -7, m = 6;
    CHECK(4, n * m == -42);              /* signed * signed (neg * pos) */
    CHECK(5, n * (-m) == 42);           /* neg * neg = pos (-7 * -6)   */

    /* constant multiplies (gcc -> shift/add strength reduction) */
    volatile int v = 12345;
    CHECK(6,  v * 8    == 98760);        /* v << 3            */
    CHECK(7,  v * 10   == 123450);       /* (v<<3) + (v<<1)   */
    CHECK(8,  v * 7    == 86415);        /* (v<<3) - v        */
    CHECK(9,  v * 1024 == 12641280);     /* v << 10           */

    /* shift edge cases: by 0, by 31, logical vs arithmetic right */
    volatile unsigned s = 1u;
    CHECK(10, (s << 31) == 0x80000000u);
    CHECK(11, ((int)0x80000000 >> 31) == -1);    /* sra: sign fill */
    CHECK(12, (0x80000000u >> 31) == 1u);        /* srl: zero fill */
    CHECK(13, (s << 0) == 1u && (s >> 0) == 1u); /* shift by 0 = identity */

    /* variable shift amount (register operand, not immediate) */
    volatile int sh = 4;
    CHECK(14, (1u << sh) == 16u && (256u >> sh) == 16u);

    return 0;
}
