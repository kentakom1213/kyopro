#             Q2-1. 表と数字 (1)
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/324

# AC
# ----------------------------------------

def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

A = list(map(int, input().split()))

DP = init_array(4, 6)
DP[0] = [0] + A + [0]

for i in range(1, 4):
    for j in range(1, 5):
        DP[i][j] = sum(DP[i-1][j-1:j+2])
    
print(DP[-1][-2])