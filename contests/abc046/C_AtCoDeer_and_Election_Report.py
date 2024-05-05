#     C - AtCoDeer and Election Report    
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc046/tasks/arc062_a
# ----------------------------------------

N = int(input())

T, A = 1, 1
for _ in range(N):
    t, a = map(int, input().split())

    n = max(
        (T + t - 1) // t,  # ceil(T / t)
        (A + a - 1) // a   # ceil(A / a)
    )

    T = n*t
    A = n*a

print(T+A)
