#              C - Vacation
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/dp/tasks/dp_c

# AC
# ----------------------------------------

# input
N = int(input())
activity = [list(map(int, input().split())) for _ in range(N)]

# solve
DP = [activity[0]] + [[0]*3 for _ in range(N-1)]

for i in range(1, N):
    for j in range(3):
        DP[i][j] = max(
            DP[i-1][(j+1)%3],
            DP[i-1][(j+2)%3]
        ) + activity[i][j]

print(max(DP[-1]))
