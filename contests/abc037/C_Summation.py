#               C - 総和
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc037/tasks/abc037_c
# ----------------------------------------

# 5 3
# 1 2 4 8 16
# -----
#   -----
#     ------

N, K = map(int, input().split())
A = list(map(int, input().split()))

ans = 0
for i in range(N):
    cnt = min(min(K, N-K+1), min(i, N-i-1) + 1)
    ans += A[i] * cnt

print(ans)
