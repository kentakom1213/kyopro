#          D - Flipping Signs
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc125/tasks/abc125_d
# 解説
# https://img.atcoder.jp/abc125/editorial.pdf

# むずい

# AC (解説)
# ----------------------------------------

# DPで解く。これができれば強い

N = int(input())
A = list(map(int, input().split()))

INF = 1e10
dp0 = [-INF for _ in range(N+1)]  # dp0[i] := i-1, i個目をひっくり返さない
dp1 = [-INF for _ in range(N+1)]  # dp1[i] := i-1, i個目をひっくり返す
dp0[0] = 0

# 更新
for i in range(N):
    dp0[i+1] = max(
        dp0[i] + A[i],  # そのまま足す
        dp1[i] - A[i]   # ひっくり返して足す
    )

    dp1[i+1] = max(
        dp0[i] - A[i],  # ひっくり返して足す
        dp1[i] + A[i]   # そのまま足す
    )

print(dp0[-1])
