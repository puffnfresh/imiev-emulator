/*
 * M32R DSP accumulator instructions, reached via inline asm (gcc never emits them
 * from C). Covers mulhi/mullo/machi/maclo (16x16 multiply[-accumulate] into the
 * 56-bit accumulator) and the mvfachi/mvfacmi/mvfaclo/mvtachi/mvtaclo moves.
 *
 * Semantics (M32R): mullo/mulhi multiply the low/high halfwords of the two source
 * registers (sign-extended) and place product<<16 in ACC; maclo/machi add to ACC.
 * mvfacmi reads ACC[16:47], mvfachi ACC[32:63], mvfaclo ACC[0:31].
 *
 * Returns 0 on success, else the id of the first failing check.
 */
#define CHECK(id, cond) do { if (!(cond)) return (id); } while (0)

/* product of low halfwords, read back as an integer via ACC[16:47] */
static inline int mullo_mid(int a, int b)
{
    int r;
    __asm__ volatile("mullo %1,%2\n\tmvfacmi %0" : "=r"(r) : "r"(a), "r"(b));
    return r;
}
/* (a.lo*b.lo) + (c.lo*d.lo), read back via ACC[16:47] */
static inline int maclo_mid(int a, int b, int c, int d)
{
    int r;
    __asm__ volatile("mullo %1,%2\n\tmaclo %3,%4\n\tmvfacmi %0"
                     : "=r"(r) : "r"(a), "r"(b), "r"(c), "r"(d));
    return r;
}
/* product of high halfwords */
static inline int mulhi_mid(int a, int b)
{
    int r;
    __asm__ volatile("mulhi %1,%2\n\tmvfacmi %0" : "=r"(r) : "r"(a), "r"(b));
    return r;
}
static inline int machi_mid(int a, int b, int c, int d)
{
    int r;
    __asm__ volatile("mulhi %1,%2\n\tmachi %3,%4\n\tmvfacmi %0"
                     : "=r"(r) : "r"(a), "r"(b), "r"(c), "r"(d));
    return r;
}
/* raw low 32 bits of ACC (= product<<16 for a small product) */
static inline unsigned mullo_lo(int a, int b)
{
    unsigned r;
    __asm__ volatile("mullo %1,%2\n\tmvfaclo %0" : "=r"(r) : "r"(a), "r"(b));
    return r;
}
/* mvtachi/mvtaclo set ACC halves; mvfachi/mvfaclo read them back */
static inline void acc_roundtrip(unsigned hi, unsigned lo, unsigned *ohi, unsigned *olo)
{
    unsigned h, l;
    __asm__ volatile("mvtachi %2\n\tmvtaclo %3\n\tmvfachi %0\n\tmvfaclo %1"
                     : "=&r"(h), "=&r"(l) : "r"(hi), "r"(lo));
    *ohi = h; *olo = l;
}

int main(void)
{
    CHECK(1, mullo_mid(3, 5) == 15);                       /* 3*5 */
    CHECK(2, maclo_mid(3, 5, 2, 7) == 29);                 /* 15 + 14 */
    CHECK(3, mulhi_mid(3 << 16, 5 << 16) == 15);           /* high halfwords */
    CHECK(4, machi_mid(3 << 16, 5 << 16, 2 << 16, 7 << 16) == 29);
    CHECK(5, mullo_mid(-3, 5) == -15);                     /* signed halfword */
    CHECK(6, mullo_lo(3, 5) == 0x000F0000u);               /* ACC[0:31] = 15<<16 */

    unsigned ohi = 0, olo = 0;
    acc_roundtrip(0x0000ABCDu, 0x12345678u, &ohi, &olo);
    CHECK(7, ohi == 0x0000ABCDu && olo == 0x12345678u);    /* mvtac/mvfac round-trip */

    return 0;
}
