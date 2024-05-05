//            A - Growth Record            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc259/tasks/abc259_a
// ----------------------------------------

#include <stdio.h>
#define max(a, b) (a > b ? a : b)

int main(void) {
    int n, m, x, t, d;
    scanf("%d%d%d%d%d", &n, &m, &x, &t, &d);

    t -= max(x-m, 0) * d;
    printf("%d\n", t);
}
