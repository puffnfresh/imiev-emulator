/*
 * Memory access widths, signed/zero extension, and BIG-ENDIAN byte order — the
 * emulator is M32R:BE:32, so byte/halfword layout is easy to get wrong. Uses volatile
 * locals so every access is a real load/store (ldb/ldub/ldh/lduh/ld, stb/sth/st).
 * Returns 0 on success, else the id of the first failing check.
 */
#define CHECK(id, cond) do { if (!(cond)) return (id); } while (0)

int main(void)
{
    /* Big-endian: most-significant byte at the lowest address. */
    volatile unsigned int w = 0x11223344u;
    volatile unsigned char *bp = (volatile unsigned char *)&w;
    CHECK(1, bp[0] == 0x11);   /* ldub */
    CHECK(2, bp[1] == 0x22);
    CHECK(3, bp[2] == 0x33);
    CHECK(4, bp[3] == 0x44);

    volatile unsigned short *hp = (volatile unsigned short *)&w;
    CHECK(5, hp[0] == 0x1122);   /* lduh, BE halfword */
    CHECK(6, hp[1] == 0x3344);

    /* signed vs unsigned byte load (ldb sign-extends, ldub zero-extends) */
    volatile signed char   sc = (signed char)0xF0;   /* -16  */
    volatile unsigned char uc = 0xF0u;               /*  240 */
    CHECK(7, (int)sc == -16);
    CHECK(8, (int)uc == 240);

    /* signed vs unsigned halfword */
    volatile short          sh = (short)0x8001;      /* -32767 */
    volatile unsigned short uh = 0x8001u;            /*  32769 */
    CHECK(9,  (int)sh == -32767);
    CHECK(10, (int)uh == 32769);

    /* store four bytes, reload as a word: BE reassembly */
    volatile unsigned int cell = 0;
    volatile unsigned char *cb = (volatile unsigned char *)&cell;
    cb[0] = 0xAA; cb[1] = 0xBB; cb[2] = 0xCC; cb[3] = 0xDD;   /* stb x4 */
    CHECK(11, cell == 0xAABBCCDDu);                            /* ld */

    /* store halfwords, reload word */
    volatile unsigned int cell2 = 0;
    volatile unsigned short *ch = (volatile unsigned short *)&cell2;
    ch[0] = 0xDEAD; ch[1] = 0xBEEF;                            /* sth x2 */
    CHECK(12, cell2 == 0xDEADBEEFu);

    /* array indexing with computed offsets + write-through pointer */
    volatile int arr[8];
    for (int i = 0; i < 8; i++) arr[i] = (i * 7) ^ 0x55;      /* st, scaled index */
    CHECK(13, arr[0] == 0x55 && arr[7] == ((7 * 7) ^ 0x55));
    volatile int *q = arr;
    *(q + 3) = 1234;
    CHECK(14, arr[3] == 1234);

    return 0;
}
