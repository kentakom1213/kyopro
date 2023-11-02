# https://atcoder.jp/contests/abc235/tasks/abc235_d

from collections import deque

a, N = map(int, input().split())

# BFS
dist = [-1] * 10_000_000
dist[N] = 0
q = deque([N])

while q:
    u = q.popleft()
    # 1/a倍
    if u % a == 0:
        v = u // a
        if dist[v] != -1:
            continue
        dist[v] = dist[u] + 1
        q.append(v)

    # 回転させる
    s = str(u)
    v = int(s[1:] + s[0])
    if dist[v] != -1:
        continue
    dist[v] = dist[u] + 1
    q.append(v)

print(dist[1])
