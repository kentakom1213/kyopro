//                 B - Mex                 
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc245/tasks/abc245_b
// ----------------------------------------

#include<stdio.h>
#include<stdlib.h>

int N, A[2020];

// 比較関数
int asc(const void *a, const void *b) {
  return *(int *)a - *(int *)b;
}

int main(void) {
    scanf("%d", &N);
    for (int i=0; i < N; i++) {
        scanf("%d", &A[i]);
    }
    qsort(A, N, sizeof(int), asc);

    int ans = 0;
    for (int i=0; i<N; i++) {
        if (A[i] == ans) {
            ans++;
        } else if (A[i] > ans) {
            printf("%d\n", ans);
            return 0;
        }
    }

    printf("%d\n", ans);
    return 0;
}
