#include<stdio.h>

#define MAX 200000

int n, q;
long s[MAX];

int main() {
    scanf("%d%d", &n, &q);

    // 累積和の計算
    long a;
    for (int i = 0; i < n; i++) {
        scanf("%ld", &a);
        s[i + 1] = s[i] + a;
    }

    // クエリの処理
    int l, r;
    for (int i = 0; i < q; i++) {
        scanf("%d%d", &l, &r);
        // 出力
        printf("%ld\n", s[r] - s[l - 1]);
    }
    
    return 0;
}
