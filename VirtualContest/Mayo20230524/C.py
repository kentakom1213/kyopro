SIZE = 20202
OFFSET = 10101
# SIZE = 20
# OFFSET = 10

N, x, y = map(int, input().split())
A = [int(v) for v in input().split()]

# 縦，横それぞれに関して部分和問題
dp = [[0] * SIZE for _ in range(N + 1)]
dp[0][OFFSET] = 1
dp[1][OFFSET + A[0]] = 1

for i in range(1, N):
    for j in range(SIZE):
        if j > A[i]:
            dp[i + 1][j - A[i]] |= dp[i - 1][j]
        if j + A[i] < SIZE:
            dp[i + 1][j + A[i]] |= dp[i - 1][j]


# 判定
is_ok = dp[N - 1][y + OFFSET] and dp[N][x + OFFSET]
print("Yes" if is_ok else "No")
