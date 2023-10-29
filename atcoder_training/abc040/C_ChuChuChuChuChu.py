#             C - 柱柱柱柱柱
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc040/tasks/abc040_c
# ----------------------------------------

INF = 1e10

N = int(input())
A = [int(n) for n in input().split()]

dp = [INF] * N  # dp[i] := 柱1から柱iに移動するときのコストの最小値
dp[0] = 0

for i in range(N-1):
    if i+1 < N:
        dp[i+1] = min(
            dp[i+1],
            dp[i] + abs(A[i] - A[i+1])
        )
    if i+2 < N:
        dp[i+2] = min(
            dp[i+2],
            dp[i] + abs(A[i] - A[i+2])
        )

print(dp[-1])
