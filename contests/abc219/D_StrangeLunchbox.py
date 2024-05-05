# D - Strange Lunchbox
# ---------------------
# 問題
# https://atcoder.jp/contests/abc219/tasks/abc219_d

# かなり難しい
# AC
# ---------------------

# 3次元DPでとく
# dp[i][j][k] := i個までの弁当の中で、j個のたこ焼きとk個のたい焼きを手に入れているときの弁当の個数

# 初期値
# dp[0][0][0] = 0
# dp[*][*][*] = INF

# 目標値
# dp[N][X][Y] を満たすNの最小値

# 更新
# (i-1, j, k) からの更新を考える
#
# 1. i個目の弁当を買うとき、A[i]個のたこ焼きとB[i]個のたい焼きが手に入る
#    ここで、弁当の個数は1増える
#    (ただし、X, Yは超えないようにする)
# 2. i個目の弁当を買わないとき、状態は(i, j, k)に変化する



INF = 1e10
MAX = 310

N = int(input())
X, Y = map(int, input().split())
A, B = [0]*N, [0]*N
for i in range(N):
    A[i], B[i] = map(int, input().split())

dp = [[[INF] * MAX for _ in range(MAX)] for _ in range(MAX)]
dp[0][0][0] = 0

for i in range(N):
    for j in range(X+1):
        for k in range(Y+1):
            if dp[i][j][k] == INF: continue

            # 買わない
            dp[i+1][j][k] = min(
                dp[i+1][j][k],
                dp[i][j][k]
            )

            # 買う
            dp[i+1][min(j+A[i], X)][min(k+B[i], Y)] = min(
                dp[i+1][min(j+A[i], X)][min(k+B[i], Y)],
                dp[i][j][k] + 1
            )

print(dp[N][X][Y] if dp[N][X][Y] < INF else -1)
