#                B - Rotate               
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc309/tasks/abc309_b
# ----------------------------------------

N = int(input())
A = [input() for _ in range(N)]

res = [[""] * N for _ in range(N)]

for i in range(N):
    for j in range(N):
        if i == 0 and j != N - 1:
            # 右にずらす
            res[i][j + 1] = A[i][j]
        elif j == N - 1 and i != N - 1:
            # 下にずらす
            res[i + 1][j] = A[i][j]
        elif i == N - 1 and j != 0:
            # 左にずらす
            res[i][j - 1] = A[i][j]
        elif j == 0 and i != 0:
            # 上にずらす
            res[i - 1][j] = A[i][j]
        else:
            # そのまま
            res[i][j] = A[i][j]
    
for r in res:
    print("".join(r))

