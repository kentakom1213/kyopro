#           D - Count Interval
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc233/tasks/abc233_d
# ----------------------------------------

# 累積和を取る
#     S[j] - S[i] == K
# <=> S[i] == S[j] - K
#
# したがって、S[j]-K の数をdictなどで保存すればいい

import sys
def err(*args, **kwargs): print(*args, **kwargs, file=sys.stderr)
from collections import defaultdict

N, K = map(int, input().split())
A = list(map(int, input().split()))

# とりあえず累積和
S = [0] * (N+1)
for i in range(N):
    S[i+1] = S[i] + A[i]

diff = defaultdict(int)
cnt = 0
for i in range(1, N+1):
    diff[S[i-1]] += 1
    cnt += diff[S[i] - K]

err(diff)

print(cnt)
