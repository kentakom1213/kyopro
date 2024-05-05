#        E - Geometric Progression        
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc293/tasks/abc293_e
# ----------------------------------------

def gcd(a, b):
    if b == 0:
        return a
    else:
        return gcd(b, a%b)

A, X, M = map(int, input().split())
A %= M

if A == 1:
    ans = (A % M) * (X % M) % M
    print(ans)
    exit()

u = (M + pow(A, X, M) - 1) % M

# A^X-1 と A-1 を約分しておく
g = gcd(A-1, M)
v = pow((A - 1) // g, -1, M // g)

ans = u * v % M
print(ans)
