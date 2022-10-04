# https://atcoder.jp/contests/abc040/tasks/abc040_c

N = int(input())
A = list(map(int, input().split()))

dp = [1e20] * N
dp[0] = 0

for i in range(N):
    if i >= 1:
        dp[i] = min(
            dp[i],
            dp[i-1] + abs(A[i] - A[i-1])
        )
    if i >= 2:
        dp[i] = min(
            dp[i],
            dp[i-2] + abs(A[i] - A[i-2])
        )

print(dp[N-1])
