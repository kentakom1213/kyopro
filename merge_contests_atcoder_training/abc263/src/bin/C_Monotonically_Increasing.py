#       C - Monotonically Increasing      
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc263/tasks/abc263_c
# ----------------------------------------

from itertools import combinations
N, M = map(int, input().split())
for comb in combinations(range(1, M+1), N):
    print(*comb)
