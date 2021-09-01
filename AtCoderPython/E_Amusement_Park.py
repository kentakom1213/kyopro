#            E - Amusement Park
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc216/tasks/abc216_e

# 参考
# https://atcoder.jp/contests/abc216/editorial/2469
# ----------------------------------------

# input
# E - Amusement Park
# TLE

from bisect import bisect_left

# input
N, K = map(int, input().split())
A = map(int, input().split())

# solve
B = sorted( sum( (list(range(1, a+1)) for a in A ), []) )

