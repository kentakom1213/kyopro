#           Q2-4. コマの移動 (1)
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/329

# AC
# ----------------------------------------

from functools import reduce
from operator import mul

N = int(input())

def fac(n): return reduce(mul, range(1, n+1))

perm = fac(2*(N-1)) // fac(N-1) ** 2

print(perm)