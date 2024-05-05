#                D - Trophy               
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc258/tasks/abc258_d
# ----------------------------------------

INF = 1e20

N, X = map(int, input().split())
A, B = [0]*N, [0]*N
for i in range(N):
    A[i], B[i] = map(int, input().split())

acc, mini, ans = 0, INF, INF
for i in range(min(X, N)):
    acc += A[i] + B[i]
    if mini > B[i]: mini = B[i]

    tmp = acc + mini * (X - i - 1)
    if ans > tmp:
        ans = tmp

print(ans)
