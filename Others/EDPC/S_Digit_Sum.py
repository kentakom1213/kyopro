#              S - Digit Sum              
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/dp/tasks/dp_s
# ----------------------------------------

MOD = 1_000_000_007

K = input()
D = int(input())

N = len(K)

# dp[i][j][k] := 上からi桁目まで見たとき、K以下であることが(j?確定:未確定)な数のうち、条件を満たすものの個数
dp = [[[0] * D for _ in range(2)] for _ in range(N + 1)]

dp[0][0][0] = 1

for i in range(N):
    k = int(K[i])
    for j in range(D):
        # 単純に桁kを足す場合
        dp[i + 1][0][(j + k) % D] += dp[i][0][j]
        dp[i + 1][0][(j + k) % D] %= MOD

        for d in range(10):
            # 確定 → 確定
            dp[i + 1][1][(j + d) % D] += dp[i][1][j]
            dp[i + 1][1][(j + d) % D] %= MOD

        for d in range(k):
            # 未確定 → 確定
            dp[i + 1][1][(j + d) % D] += dp[i][0][j]
            dp[i + 1][1][(j + d) % D] %= MOD

ans = (MOD + dp[N][0][0] + dp[N][1][0] - 1) % MOD

print(ans)
