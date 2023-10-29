#include<stdio.h>

#define MAX 200000

int n, q;
long a[MAX];

int main() {
    scanf("%d%d", &n, &q);

    // aを入力
    for (int i = 0; i < n; i++) {
        scanf("%ld", &a[i]);
    }

    // クエリの処理
    int l, r;
    for (int i = 0; i < q; i++) {
        scanf("%d%d", &l, &r);
        
        // 合計を計算する
        long ans = 0;
        for (int j = l - 1; j < r; j++) {
            ans += a[j];
        }

        printf("%ld\n", ans);
    }
    
    return 0;
}
