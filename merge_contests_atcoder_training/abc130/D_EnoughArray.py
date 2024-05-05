#           D - Enough Array
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc130/tasks/abc130_d
# ----------------------------------------

N, K = map(int, input().split())
A = list(map(int, input().split()))

# K未満である区間の個数を尺取法で求める
summ = 0
ans = 0

r = 0
for l in range(N):
    while r < N and summ + A[r] < K:
        summ += A[r]
        r += 1
    
    ans += N - r
    
    if l < r:
        summ -= A[l]
    elif l == r:
        r += 1

print(ans)