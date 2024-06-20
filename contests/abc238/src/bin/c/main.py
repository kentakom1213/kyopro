
MOD = 998244353

N = int(input())
D = len(str(N)) - 1

# 桁ごとに考える
ans = 0
for i in range(0, D):
    max_ = 10**(i+1) - 10**i
    ans += (max_ + 1) * max_ // 2
    ans %= MOD

# 1番大きい桁
ans += (N - 10**D + 2) * (N - 10**D + 1) // 2

print(ans % MOD)