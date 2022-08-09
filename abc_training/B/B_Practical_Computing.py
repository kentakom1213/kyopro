#         B - Practical Computing         
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc254/tasks/abc254_b
# ----------------------------------------

N = int(input())
A = [[0]*N for _ in range(N)]

for i in range(N):
    for j in range(i+1):
        if j == 0 or i == j:
            A[i][j] = 1
        else:
            A[i][j] = A[i-1][j-1] + A[i-1][j]

for i in range(N):
    print(*A[i][:i+1])
