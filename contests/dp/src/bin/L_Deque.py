#                L - Deque
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/dp/tasks/dp_l
# ----------------------------------------

import sys
sys.setrecursionlimit(10000000)

INF = 1e20

N = int(input())
A = list(map(int, input().split()))

# dp[i][j] := 先頭がi、末尾がjであるときのスコアの
#             最大値（太郎）、最小値（次郎）
dp = [[INF] * (N + 1) for _ in range(N + 1)]


def rec(l, r):
    if dp[l][r] != INF:
        return dp[l][r]

    if l == r:
        return 0

    # 現状のプレイヤー視点で、最大となる手
    res = max(
        -rec(l + 1, r) + A[l],  # 先頭を選択
        -rec(l, r - 1) + A[r - 1]  # 末尾を選択
    )

    dp[l][r] = res
    return res


ans = rec(0, N)

print(ans)
