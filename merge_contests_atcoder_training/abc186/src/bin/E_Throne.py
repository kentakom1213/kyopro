#                E - Throne
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc186/tasks/abc186_e
# ----------------------------------------

from math import gcd
t = int(input())
for _ in range(t):
    n, s, k = map(int, input().split())
    g = gcd(gcd(n, s), k)
    n //= g
    s //= g
    k //= g
    if gcd(k, n) == 1:
        print((n-s)*pow(k, -1, n) % n)
    else:
        print(-1)
