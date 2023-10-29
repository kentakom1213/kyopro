#         C - Linear Approximation        
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc102/tasks/arc100_a
# ----------------------------------------

import numpy as np

N = int(input())
A = np.array([int(v)for v in input().split()]) - np.arange(0, N)

# 中央値
med = int(np.median(A))

# 合計
ans = np.abs(A - med).sum()

print(ans)
