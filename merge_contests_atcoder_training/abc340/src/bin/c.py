import sys
sys.setrecursionlimit(1000000)

from functools import lru_cache

N = int(input())

@lru_cache
def rec(n):
    if n < 2:
        return 0
    return n + rec(n // 2) + rec((n + 1) // 2)

print(rec(N))
