
MOD = 998244353

def inv(n):
    """フェルマーの小定理を用いて逆元を求める"""
    return pow(n, MOD-2, MOD)


# input
N, Q = map(int, input().split())
A = list(map(int, input().split()))

# 累積積を求める
S = [1] * (N+1)
for i in range(N):
    S[i+1] = S[i] * A[i] % MOD

# クエリの処理
for _ in range(Q):
    l, r = map(int, input().split())
    l -= 1

    ans = S[r] * inv(S[l]) % MOD
    print(ans)
