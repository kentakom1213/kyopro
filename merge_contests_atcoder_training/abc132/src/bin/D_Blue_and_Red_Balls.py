# D - Blue and Red Balls
# -----------------------
# 問題
# https://atcoder.jp/contests/abc132/tasks/abc132_d
# -----------------------

MOD = 10**9+7
MAX = 4100

# 二項係数を求める
C = [[0]*(MAX+1) for _ in range(MAX+1)]
C[0][0] = 1

for i in range(MAX):
    for j in range(MAX):
        C[i+1][j] = (C[i+1][j] + C[i][j]) % MOD
        C[i+1][j+1] = (C[i+1][j+1] + C[i][j]) % MOD

def nHr(n, r):
    return C[n+r-1][r]

# solve
N, K = map(int, input().split())

for i in range(1, K+1):
    b = K - i
    r = N-K - (i-1)

    ans = nHr(i, b) * nHr(i+1, r)
    ans %= MOD
    print(ans)
