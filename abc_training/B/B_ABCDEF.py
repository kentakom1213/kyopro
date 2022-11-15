MOD = 998244353
a, b, c, d, e, f = map(int, input().split())

abc = a%MOD * b%MOD * c%MOD
def_ = d%MOD * e%MOD * f%MOD
ans = (abc - def_) % MOD

print(ans)
