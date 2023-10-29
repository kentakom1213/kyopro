#       044 - Shift and Swapping（★3）
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/typical90/tasks/typical90_ar

# AC
# ----------------------------------------

# 愚直にシフト O(n) をしていると間に合わない
# シフトをする代わりに配列の最初のインデックスの変数(s)を変化させて対応する。
# インデックスの対応は A[(s+i)%N]

N, Q = map(int, input().split())
A = list(map(int, input().split()))
queries = [tuple(map(int, input().split())) for _ in range(Q)]

s = 0
for t, x, y in queries:
    x -= 1
    y -= 1
    nx, ny = (x-s) % N, (y-s) % N
    if t == 1:
        A[nx], A[ny] = A[ny], A[nx]
    elif t == 2:
        s = (s + 1) % N
    else:
        print(A[nx])
    
    # for i in range(N):
    #     print(A[(i-s) % N], end=" ")
    # print()
