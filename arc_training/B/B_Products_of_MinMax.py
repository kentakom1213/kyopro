#         B - Products of Min-Max         
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc116/tasks/arc116_b
# ----------------------------------------

mod = 998244353

N = int(input())
A = list(map(int, input().split()))

A.sort()

# O(n^2)に落とす
ans = 0
for i in range(N):
    for j in range(i, N):
        cnt = 1 << max(0, j-i-1)
        ans += A[i] * A[j] * cnt
        ans %= mod

print(ans % mod)
