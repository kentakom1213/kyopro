
from collections import deque


N = int(input())
A = list(map(int, input().split()))

Q = int(input())
queries = [0] * Q
qs = []
for i in range(Q):
    l, r, x = map(int, input().split())
    queries[i] = l-1, r, x
    qs.append((l-1, x))
    qs.append((r, x))

qs.sort()
q = deque(qs)

# save[(i, j)] := A0 ~ Aiまでのjの個数
acc_cnt = {}
cnt = [0] * (N+10)

while q and q[0][0] == 0:
    v, x = q.popleft()
    acc_cnt[(0, x)] = 0

for i, a in enumerate(A):
    cnt[a] += 1
    while q and q[0][0] == i+1:
        v, x = q.popleft()
        acc_cnt[(v, x)] = cnt[x]

for l, r, x in queries:
    res = acc_cnt[(r, x)] - acc_cnt[(l, x)]
    print(res)
