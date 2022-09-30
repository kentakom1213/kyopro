#            D - Non-decreasing           
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc081/tasks/arc086_b
# ----------------------------------------

N = int(input())
A = list(map(int, input().split()))

# 最小値、最大値とそのインデックス
minA = 1e20
mini = 0
maxA = -1e20
maxi = 0
for i in range(N):
    if maxA < A[i]:
        maxA = A[i]
        maxi = i + 1
    if minA > A[i]:
        minA = A[i]
        mini = i + 1

if minA >= 0:
    # 累積和をとる
    print(N-1)
    for i in range(1, N):
        print(i, i+1)
    exit()

if maxA <= 0:
    # 逆向きに累積和を取る
    print(N-1)
    for i in range(N, 1, -1):
        print(i, i-1)
    exit()

if abs(minA) <= maxA:
    # 全体を最大値で正の値にする
    print(2*N-2)
    for i in range(1, N+1):
        if i != maxi:
            print(maxi, i)

    # 累積和
    for i in range(1, N):
        print(i, i+1)
else:
    # 全体を最小値で負の値にする
    print(2*N-2)
    for i in range(1, N+1):
        if i != mini:
            print(mini, i)
    
    # 逆向きの累積和
    for i in range(N, 1, -1):
        print(i, i-1)