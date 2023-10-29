import sys
sys.setrecursionlimit(10000000)

MOD = 998244353

N, X = map(int, input().split())
T = list(map(int, input().split()))
memo = [-1] * 10101

def f(x):
    """時刻xで音楽が再生されていない確率"""
    if memo[x] != -1:
        return memo[x]
    if x == 0:
        memo[x] = 1
        return memo[x]
    ans = 0
    for t in T:
        if x - t >= 0:
            ans += f(x - t) * pow(N, -1, MOD)
            ans %= MOD
    memo[x] = ans
    return memo[x]

ans = 0
for t in reversed(range(max(0, X - T[0] + 1), X + 1)):
    ans += f(t)
    ans %= MOD

ans *= pow(N, -1, MOD)
ans %= MOD

print(ans)
