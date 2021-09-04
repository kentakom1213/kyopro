#              B - 高橋君ゲーム
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc057/tasks/arc057_b

# ARCはむずい
# ----------------------------------------

def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

N, K = map(int, input().split())
a = [int(input()) for _ in range(N)]

INF = 1e10
dp = init_array(N+1, K, INF)
dp[1][1] = 1

for i in range(N):
    for j in range(K):
        dp[i+1][j] = min(
            dp[i+1][j],

        )
