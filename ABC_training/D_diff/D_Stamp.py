#               D - Stamp
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc185/tasks/abc185_d

# こういう問題を速解きできるように

# AC
# ----------------------------------------
from math import ceil

n, m = map(int, input().split())
A = [-1] + [int(a) - 1 for a in input().split()]

A.sort()
A.append(n)

diff = [A[i+1] - A[i] - 1 for i in  range(len(A)-1) if A[i+1] - A[i] > 1]

if diff:
    stamp_size = min(diff)
    res = sum( ceil(seg / stamp_size)  for seg in diff )
    print(res)
else:
    print(0)