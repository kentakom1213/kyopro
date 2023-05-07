#               A - Flog 1
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/dp/tasks/dp_a

# AC
# ----------------------------------------

# input
N = int(input())
h = list(map(int, input().split()))

INF = 1e10
DP = [INF] * N

# DPテーブルの初期化
DP[0] = 0
DP[1] = abs(h[1] - h[0])

for i in range(N-2):
    DP[i+2] = min(
        DP[i] + abs(h[i+2] - h[i]),
        DP[i+1] + abs(h[i+2] - h[i+1])
    )

print(DP[-1])
