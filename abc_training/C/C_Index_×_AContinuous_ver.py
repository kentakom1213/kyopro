#      C - Index × A(Continuous ver.)     
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc267/tasks/abc267_c
# ----------------------------------------

N, M = map(int, input().split())
A = list(map(int, input().split()))

# 累積和
S = [0] * (N+1)
for i in range(N):
    S[i+1] = S[i] + A[i]

# 初期化
now = 0 # 1 ~ M までの和
for i in range(M):
    now += A[i] * (i+1)

# O(1) で計算
ans = now
for i in range(N-M):
    now = now - (S[i+M] - S[i]) + A[i+M] * M
    if ans < now:
        ans = now

print(ans)
