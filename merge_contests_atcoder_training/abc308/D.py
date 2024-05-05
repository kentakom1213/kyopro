from collections import deque


def is_next(c, d):
    if c not in SNUKE or d not in SNUKE:
        return False
    else:
        cidx = SNUKE.index(c)
        didx = SNUKE.index(d)
        return (cidx + 1) % 5 == didx


INF = 1e20
SNUKE = "snuke"
MOVE = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
]

H, W = map(int, input().split())
S = [input() for _ in range(H)]

G = [[] for _ in range(H * W)]

# グラフの初期化
for r in range(H):
    for c in range(W):
        for dr, dc in MOVE:
            nr, nc = r + dr, c + dc
            if 0 <= nr < H and 0 <= nc < W and is_next(
                S[r][c], S[nr][nc]
            ):
                cur = r * W + c
                nxt = nr * W + nc
                G[cur].append(nxt)

# BFS
dist = [INF] * (H * W)
dist[0] = 0
que = deque([0])

# bfs
while que:
    u = que.popleft()
    for v in G[u]:
        if dist[v] == INF:
            que.append(v)
            dist[v] = dist[u] + 1

print("Yes" if dist[H * W - 1] < INF else "No")
