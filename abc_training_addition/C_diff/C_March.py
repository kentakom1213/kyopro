#               C - March
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc089/tasks/abc089_c
# ----------------------------------------

# 重複組合せ

from functools import reduce
from operator import mul
factorial = lambda x: reduce(mul, range(1, x+1))


if __name__ == "__main__":
    N = int(input())
    
    names = {}
    for _ in range(N):
        name = input()

        if name[0] in "MARCH":
            if name in names:
                names[name] = 1
            else:
                names[name] += 1
    
    res = factorial(len(names))
    
    for v in names.values():
        res //= v
    
    print(res)