#                I - Coins
# ----------------------------------------
# 問題
# https:#atcoder.jp/contests/dp/tasks/dp_i
# ----------------------------------------

# dp[i][j] := i枚目までのコインを投げた時に

def exprint(x): print(*x, sep="\n")
def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

N = int(input())
coins = list(map(int, input().split()))

dp = init_array(N, N+1, 0)

