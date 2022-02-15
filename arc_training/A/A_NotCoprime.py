#           A - Not coprime
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc114/tasks/arc114_a
# ----------------------------------------

# 50までの素数は15個であるから、bit全探索によって最小値を求めればよい

import sys
def err(*args, **kwargs): print(*args, **kwargs, file=sys.stderr)
ceil = lambda x: int( - (-x // 1) )
from functools import reduce
from operator import mul

primes = (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47,)
p = len(primes)

N = int(input())
X = list(map(int, input().split()))

# 全て素因数分解
def factorize(n):
    res = set()
    for i in range(2, ceil(pow(n, 1/2))+1):
        while n % i == 0:
            res.add(i)
            n //= i
    if n != 1: res.add(n)
    return res

factors = [factorize(x) for x in X]

# bit全探索
mini = reduce(mul, primes)
for i in range(1 << p):
    # 選び方
    choose = set(primes[j] for j in range(p) if (i >> j) & 1)
    
    isOK = True
    for fac in factors:
        isOK &= bool(fac & choose)  # 共通部分をもつか判定
    
    if isOK:
        mini = min(mini, reduce(mul, choose))
    
print(mini)
