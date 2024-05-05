#             D - XOR World
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc121/tasks/abc121_d
# ----------------------------------------

# 任意の偶数nについて
# n ^ (n+1) = 1

A, B = map(int, input().split())

def f(x):
    """0からxまでの排他的論理和を求める"""
    if x & 1:
        return (x >> 1) & 1
    else:
        return (((x-1) >> 1) & 1) ^ x

ans = f(A-1) ^ f(B)
print(ans)
