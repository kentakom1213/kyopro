#          0-1 Knapsack Problem
# ----------------------------------------
# 問題
# https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_B&lang=jp

# 参考
# https://oretano.com/dynamic-programming
# DPがよく分かっていないので、その練習として。

# むずいのでとりあえずパス
# ----------------------------------------

def mapl(func, iter): return list(map(func, iter))

# input
N, W = map(int, input().split())
obj = [mapl(int, input().split()) for _ in range(N)]

# solve
dp = [[] for _ in range()]
