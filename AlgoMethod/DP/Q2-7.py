#           Q2-6. コマの移動 (4)
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/335

# AC
# ----------------------------------------

INF = 1e10
def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

N = int(input())
A = [list(map(int, input().split()))[::-1] for _ in range(N)]

for i in range(N):
    for j in range(N):
        if (i, j) == (0, 0): continue

        A[i][j] = min(
            A[i-1][j] if i else INF,
            A[i][j-1] if j else INF
        ) + A[i][j]

# print(*A, sep="\n")
print(A[-1][-1])