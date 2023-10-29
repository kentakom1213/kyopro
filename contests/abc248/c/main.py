
mod = 998244353

# comb
n = 100000
fac = [1] * (n + 1)
inv = [1] * (n + 1)
for j in range(1, n + 1):
    fac[j] = fac[j-1] * j % mod


inv[n] = pow(fac[n], mod-2, mod)
for j in range(n-1, -1, -1):
    inv[j] = inv[j+1] * (j+1) % mod


def comb(n, r):
    if r > n or n < 0 or r < 0:
        return 0
    return fac[n] * inv[n - r] * inv[r] % mod

# input
N, M, K = map(int, input().split())

ans = 0
for k in range(1, K+1):
    res = comb(k-1, k-N)

    if k > M:
        res -= comb(k-M-1, k-M-N)
    
    ans += res
    ans %= mod

print(ans)
