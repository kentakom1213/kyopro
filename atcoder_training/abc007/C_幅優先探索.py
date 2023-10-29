#                C - 幅優先探索                
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc007/tasks/abc007_3
# ----------------------------------------

from collections import deque

MOVE = [
    (1, 0),
    (-1, 0),
    (0, -1),
    (0, 1),
]

R, C = map(int, input().split())
sr, sc = map(int, input().split())
gr, gc = map(int, input().split())
sr -= 1
sc -= 1
gr -= 1
gc -= 1

F = [input() for _ in range(R)]

q = deque([(sr, sc)])
dist = [[1e20] * C for _ in range(R)]
dist[sr][sc] = 0

while q:
    cr, cc = q.popleft()
    for dr, dc in MOVE:
        nr = cr + dr
        nc = cc + dc
        if F[nr][nc] == "#" or dist[nr][nc] < 1e20:
            continue
        dist[nr][nc] = dist[cr][cc] + 1
        q.append((nr, nc))

print(dist[gr][gc])
