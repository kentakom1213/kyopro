# https://atcoder.jp/contests/abc235/tasks/abc235_d

from collections import deque

a, N = map(int, input().split())

# BFS
dist = [-1] * 10_000_000
dist[1] = 0
q = deque([1])

while q:
    u = q.popleft()
    # a倍
    v = u * a
    if v < 10_000_000 and dist[v] == -1:
        dist[v] = dist[u] + 1
        q.append(v)
    # 回転させる
    if u >= 10 and u % 10 != 0:
        s = str(u)
        v = int( s[-1] + s[:-1] )
        if dist[v] == -1:
            dist[v] = dist[u] + 1
            q.append(v)

print(dist[N])
