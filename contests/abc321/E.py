"""
- 部分木の根からの高さを求めていくのが良さそう。
- N <= 1e18より、木の高さは高々64
- 頂点間の距離は高々128
"""

from functools import lru_cache

@lru_cache
def f(n, x, k):
    # 左の子について
    cl = x << 1

    # 右の子について
    cr = cl + 1

T = int(input())
tests = [tuple(map(int, input().split())) for _ in range(T)]

for n, x, k in tests:
    print(f(n, x, k))
