
import sys
input = sys.stdin.readline

# input
N, Q = map(int, input().split())
A = list(map(int, input().split()))

# 累積和を求める
S = [0] * (N+1)
for i in range(N):
    S[i+1] = S[i] + A[i]

# クエリの処理
for _ in range(Q):
    l, r = map(int, input().split())
    l -= 1

    ans = S[r] - S[l]
    print(ans)
