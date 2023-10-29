
ceil = lambda x: int(-(-x//1))
from bisect import bisect_left
from array import array

N = int(input())
P = list(map(int, input().split()))
Q = list(map(int, input().split()))

# Bの逆表を作る
ind = [0] * (N+1)
for i in range(N):
    ind[Q[i]] = i

# 約数列挙
pairs = []
for i, p in enumerate(P):
    for k in range(1, ceil(N/p)+1):
        if p*k <= N:
            j = ind[p*k]
            pairs.append((i, j))

# (i, -j)をキーにソート
pairs.sort(key=lambda a: (a[0], -a[1]))

# jに関してLIS
# 同じiに対してjは降順であるため、iは狭義単調増加となる
dp = [1e10] * N
for i, j in pairs:
    ins = bisect_left(dp, j)
    dp[ins] = j

i = 0
while i < N and dp[i] < 1e10:
    i += 1

print(i)
