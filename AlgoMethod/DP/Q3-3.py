#       Q3-3. ナップサック問題 (導入編)
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/341

# 漸化式はあっていたけど、更新が完全ではなかった

# AC (解説)
# ----------------------------------------

def exprint(x): print(*x, sep="\n")
def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

N, M = map(int, input().split())
A = list(map(int, input().split()))
B = list(map(int, input().split()))

DP = init_array(N, M, -1)
DP[0][0] = 0

for i in range(N-1):
    for j in range(M):
        # 行く可能性がない場合
        if DP[i][j] < 0:
            continue

        # 真下にわたす
        DP[i+1][j] = max(DP[i+1][j], DP[i][j])

        if j + A[i] < M:
            DP[i+1][j + A[i]] = max(
                DP[i+1][j + A[i]],
                DP[i][j] + B[i]
            )

        
print(DP[-1][-1])