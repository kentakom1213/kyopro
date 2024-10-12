n, k = map(int, input().split())
AB = []

for _ in range(n):
    ai, bi = map(int, input().split())
    AB.append((ai, bi))

AB.sort(key=lambda x: (x[0] - 1) / x[1])
print(AB)

dp = [[int(-1e9)] * (k + 1) for _ in range(n + 1)]
dp[0][0] = 1

for i, (a, b) in enumerate(AB):
    for j in range(k):
        dp[i + 1][j + 1] = max(dp[i][j + 1], dp[i][j] * a + b)

print(dp[n][k])
