#              Q3-2. 部分和問題
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/337

# 解説をみました
# (N+1) * (M+1) の配列を用意する -> DP[0][0] = 1と初期化

# AC
# ----------------------------------------

def exprint(x): print(*x, sep="\n")
def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

# input
N, M = map(int, input().split())
W = list(map(int, input().split()))

DP = init_array(N+1, M+1)
DP[0][0] = 1

for i in range(N):
    for j in range(M+1):
        DP[i+1][j] += DP[i][j]

        if j + W[i] <= M:
            DP[i+1][j + W[i]] += DP[i][j]

# exprint(DP)

if DP[-1][-1]:
    print("Yes")
else:
    print("No")

# 参考演算子を使って綺麗に
# print("Yes" if DP[-1][-1] else "No")