#       D - Count Bracket Sequences       
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc312/tasks/abc312_d
# ----------------------------------------

MOD = 998244353
S = input()
N = len(S)

# dp[i][j] := S[:i]までの'?'を埋めたとき、
#             '('の数 - ')'の数がj個であるような文字列の個数
dp = [[0] * (N + 1) for _ in range(N + 1)]
dp[0][0] = 1

for i, c in enumerate(S):
    for j in range(N):
        if c in "(?":
            dp[i + 1][j + 1] += dp[i][j]
            dp[i + 1][j + 1] %= MOD
        if c in ")?":
            if j > 0:
                dp[i + 1][j - 1] += dp[i][j]
                dp[i + 1][j - 1] %= MOD

print(dp[N][0])
