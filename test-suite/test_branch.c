/*
 * Control flow: function calls / recursion (bl + jmp lr, stack frames), loops and
 * conditional branches, and a switch (jump table in rodata -> indirect jmp).
 * Returns 0 on success, else the id of the first failing check.
 */
#define CHECK(id, cond) do { if (!(cond)) return (id); } while (0)

static int fact(int n) { return n <= 1 ? 1 : n * fact(n - 1); }
static int fib(int n)  { return n < 2 ? n : fib(n - 1) + fib(n - 2); }

/* Dense cases => gcc -O2 usually emits a jump table (tests indirect branch). */
static int classify(int x)
{
    switch (x) {
    case 0:  return 100;
    case 1:  return 101;
    case 2:  return 102;
    case 3:  return 103;
    case 4:  return 104;
    default: return 999;
    }
}

int main(void)
{
    CHECK(1, fact(5) == 120);   /* call/return + stack frame linkage */
    CHECK(2, fib(10) == 55);    /* deeper recursion, two calls per frame */

    volatile int sum = 0;
    for (volatile int i = 0; i < 100; i++) sum += i;   /* loop + signed compare */
    CHECK(3, sum == 4950);

    int p = 1;
    for (int i = 0; i < 10; i++) p += p;               /* doubling: 2^10 */
    CHECK(4, p == 1024);

    CHECK(5, classify(0) == 100);
    CHECK(6, classify(4) == 104);
    CHECK(7, classify(9) == 999);   /* default arm */

    volatile int a = 5, b = -3;                        /* logical short-circuit */
    CHECK(8, (a > 0 && b < 0) && !(a < 0 || b > 0));

    /* do-while with continue (skip 3) and break (stop at 7): 0+1+2+4+5+6 = 18 */
    int cnt = 0, i = 0;
    do {
        if (i == 3) { i++; continue; }
        if (i == 7) break;
        cnt += i; i++;
    } while (i < 100);
    CHECK(9, cnt == 18);

    return 0;
}
