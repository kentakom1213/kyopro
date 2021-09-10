#           Q2-5. コマの移動 (2)
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/333

# AC
# ----------------------------------------

def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

N = int(input())

field = [input() for _ in range(N)]

DP = init_array(N, N)
DP[0][0] = 1

for i in range(N):
    for j in range(N):
        if (i, j) == (0, 0):
            pass
        if field[i][j] == "#":
            continue

        if i > 0:
            DP[i][j] += DP[i-1][j]
        if j > 0:
            DP[i][j] += DP[i][j-1]

print(DP[-1][-1])