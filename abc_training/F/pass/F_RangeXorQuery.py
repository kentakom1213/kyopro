# F - Range Xor Query
# --------------------
# 問題
# https://atcoder.jp/contests/abc185/tasks/abc185_f
# --------------------

# xorについてよくわかっていないので一旦愚直実装

from operator import xor
from functools import reduce

N, Q = map(int, input().split())
A = list(map(int, input().split()))
queries = [tuple(map(int, input().split())) for _ in range(Q)]

for t, x, y in queries:
    x -= 1
    if t == 1:
        A[x] = A[x] ^ y
    else:
        y -= 1
        print(reduce(xor, A[x:y+1]))

