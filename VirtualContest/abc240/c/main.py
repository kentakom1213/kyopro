
N, X = map(int, input().split())

dp = [[0]*(X+1) for _ in range(N+1)]
dp[0][0] = 1

for i in range(N):
    a, b = map(int, input().split())
    for j in range(X):
        if dp[i][j] == 0:
            continue
        if j+a <= X:
            dp[i+1][j+a] = 1
        if j+b <= X:
            dp[i+1][j+b] = 1

print("Yes" if dp[N][X] else "No")
