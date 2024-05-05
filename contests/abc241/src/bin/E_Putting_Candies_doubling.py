#           E - Putting Candies           
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc241/tasks/abc241_e
# ----------------------------------------

"""

## 方針
- ダブリング

"""


n, k = map(int, input().split())
a = list(map(int, input().split()))

dp = [[0]*n for _ in range(40)]

# 前計算
for j in range(n):
    dp[0][j] = a[j]

for i in range(39):
    for j in range(n):
        dp[i+1][j] = dp[i][j] + dp[i][(j + dp[i][j]) % n]  # 2倍進める

ans = 0
for i in range(40):
    if k & 1:
        ans += dp[i][ans%n]
    k >>= 1

print(ans)
