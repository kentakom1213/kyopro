// C - Otoshidama
// ---------------
// 問題
// https://atcoder.jp/contests/abc085/tasks/abc085_c
//
// AC
// ---------------

// N<=2000より、O(n^2)であれば間に合う

#include <iostream>
using namespace std;

int main() {
    int N, Y; cin >> N >> Y;
    for (int i = 0; i <= N; i++) {
        for (int j = 0; j <= N-i; j++) {
            int k = N - i - j;
            if (10000*i + 5000*j + 1000*k == Y) {
                printf("%d %d %d\n", i, j, k);
                return 0;
            }
        }
    }
    printf("-1 -1 -1\n");
}
