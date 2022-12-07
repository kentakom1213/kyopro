# https://atcoder.jp/contests/keyence2021/tasks/keyence2021_b

from heapq import heapify, heappop

N, K = map(int, input().split())
A = list(map(int, input().split()))

cnt = [0] * 303030

for a in A:
    cnt[a] += 1

now = K
ans = 0
for i in range(N):
    now = min(now, cnt[i])
    ans += now

print(ans)
