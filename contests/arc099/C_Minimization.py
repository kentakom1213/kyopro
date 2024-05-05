#            C - Minimization
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc099/tasks/arc099_a
# ----------------------------------------

from collections import Counter
div_ceil = lambda a, b: -int(-a // b)

N, K = map(int, input().split())
A = list(map(int, input().split()))

cnt = Counter(A)
mini = min(cnt)

ans = div_ceil(N, )
