#        C - Boxes and Candies
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc048/tasks/arc064_a

# コーナーケースに気をつけて
# ----------------------------------------

N, x = map(int, input().split())
A = list(map(int, input().split()))

ans = 0

# A[0] > xの場合
if A[0] > x:
    ans += A[0] - x
    A[0] = x

# 貪欲法
for i in range(N-1):
    sup = max(0, A[i] + A[i+1] - x)
    ans += sup
    A[i+1] -= sup

print(ans)
