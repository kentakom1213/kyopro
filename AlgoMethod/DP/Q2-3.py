#             Q2-3. 3 つの仕事
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/41

# AC
# ----------------------------------------

def init_array(i, j, val=0): return [[val]*j for _ in range(i)]


N = int(input())
wages = [list(map(int, input().split())) for _ in range(N)]

DP = init_array(N, 3)
DP[0] = wages[0]

for i in range(1, N):
    for j in range(3):
        DP[i][j] = max(
            DP[i-1][(j+1)%3],
            DP[i-1][(j+2)%3]
        ) + wages[i][j]

# print(*DP, sep="\n")
print(max(DP[-1]))