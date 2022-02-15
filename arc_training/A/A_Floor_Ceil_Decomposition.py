#     A - Floor, Ceil - Decomposition
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc135/tasks/arc135_a
# ----------------------------------------

import sys
sys.setrecursionlimit(10000)
from functools import lru_cache
MOD = 998244353

@lru_cache
def decompose(x):
    if x < 5:
        return x
    f, c = x//2, -int(-x//2)
    return decompose(f) * decompose(c) % MOD

if __name__ == "__main__":
    X = int(input())
    ans = decompose(X)
    print(ans)
