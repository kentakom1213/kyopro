#             D - Bouquet
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc156/tasks/abc156_d
# ----------------------------------------

"""comment
- comb(N, r) を求める必要がある。
- Nは大きいが、rが小さいため愚直に計算しても間に合う

comb(n, r) = n*(n-1)*(n-2)*...*(n-r+1) / r*(r-1)*(r-2)*...*1
"""

mod = 1000000007

n, a, b = map(int, input().split())

def mod_pow(long a, long b, long p):
    cdef long res = 1
    while b:
        if b & 1:
            res = (res * a) % p
        a = (a * a) % p
        b >>= 1
    return res

def mod_inv(long x, long p):
    return mod_pow(x, p-2, p)

def mod_comb(long n, long r, long p):
    cdef long res = 1
    for i in range(r):
        res *= n - i
        res *= mod_inv(i + 1, p)
        res %= p
    return res


ans = mod_pow(2, n, mod) - mod_comb(n, a, mod) - mod_comb(n, b, mod) - 1
ans = (ans + mod) % mod
print(ans)
