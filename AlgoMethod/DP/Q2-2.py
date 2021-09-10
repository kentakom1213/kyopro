#            Q2-2. 表と数字 (2)
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/325

# AC
# ----------------------------------------

def init_array(i, j, val=0): return [[val]*j for _ in range(i)]


N = int(input())
A = list(map(int, input().split()))

DP = init_array(N, N+2)
DP[0] = [0] + A + [0]

for i in range(1, N):
    for j in range(1, N+1):
        DP[i][j] = sum(DP[i-1][j-1:j+2]) % 100

print(DP[-1][-2])