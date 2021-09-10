#           Q2-6. コマの移動 (3)
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/334

# AC
# ----------------------------------------

def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

N = int(input())
A = [list(map(int, input().split())) for _ in range(N)]

DP = init_array(N, N)

for i in range(N):
    for j in range(N):
        DP[i][j] = max(
            DP[i-1][j] if i else 0,
            DP[i][j-1] if j else 0
        ) + A[i][j]

# print(*DP, sep="\n")
print(DP[-1][-1])