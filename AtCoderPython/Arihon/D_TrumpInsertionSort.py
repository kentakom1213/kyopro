#           D - トランプ挿入ソート
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc006/tasks/abc006_4

# 意外にサラッと解けた。LISの類題かな

# AC
# ----------------------------------------

from bisect import bisect_left

N = int(input())
cards = [int(input()) for _ in range(N)]

INF = 1e10
dp = [INF] * N

counter = 0
for c in cards:
    i = bisect_left(dp, c)

    if dp[i] != INF:
        counter += 1

    dp[i] = c

print(counter)
