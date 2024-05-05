
N, x, y = map(int, input().split())
A = list(map(int, input().split()))

IDX_MAX = 40404
OFFSET = 20202

dp = [[0]*IDX_MAX for _ in range(N+1)]

dp[0][OFFSET] = 1  # y座標の初期値をセット
dp[1][OFFSET + A[0]] = 1  # x座標の初期値をセット

# 初期値を省く
A = A[1:]

for i in range(0, N-1):
    for j in range(IDX_MAX):
        if dp[i][j]:
            dp[i+2][j + A[i]] = 1
            dp[i+2][j - A[i]] = 1


# for i in range(N+1):
#     print("yx"[i&1], dp[i])

# 判定
if N % 2 == 0:
    is_x = dp[-2][x + OFFSET]
    is_y = dp[-1][y + OFFSET]
else:
    is_x = dp[-1][x + OFFSET]
    is_y = dp[-2][y + OFFSET]

if is_x and is_y:
    print("Yes")
else:
    print("No")
