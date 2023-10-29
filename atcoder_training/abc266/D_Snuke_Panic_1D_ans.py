#           D - Snuke Panic (1D)          
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc266/tasks/abc266_d

# 解説
# https://atcoder.jp/contests/abc266/editorial/4661
# ----------------------------------------

# 方針
# 遷移は1秒単位で考えれば良い

from collections import defaultdict

MAX_T = 101010
N = int(input())

dp = [[-1]*5 for _ in range(MAX_T)]
dp[0][0] = 0  # t=0 では x=0

sunuke = defaultdict(int)

# 入力
for _ in range(N):
    t, x, a = map(int, input().split())
    sunuke[(t, x)] = a

# 遷移
for i in range(1, MAX_T):
    for j in range(5):
        if dp[i-1][j] != -1:
            dp[i][j] = max(
                dp[i][j],
                dp[i-1][j] + sunuke[(i, j)]
            )

        if j > 0 and dp[i-1][j-1] != -1:
            dp[i][j] = max(
                dp[i][j],
                dp[i-1][j-1] + sunuke[(i, j)]
            )
        
        if j < 4 and dp[i-1][j+1] != -1:
            dp[i][j] = max(
                dp[i][j],
                dp[i-1][j+1] + sunuke[(i, j)]
            )

print(max(dp[-1]))
