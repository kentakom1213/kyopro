# 043 - Maze Challenge with Lack of Sleep（★4）
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/typical90/tasks/typical90_aq
# ----------------------------------------

# 01-BFS

from collections import deque

INF = 1e20
AROUND = [(0, 1), (1, 0), (0, -1), (-1, 0)]  # [right, down, left, up]

H, W = map(int, input().split())
sr, sc = map(int, input().split()); sr-=1; sc-=1
tr, tc = map(int, input().split()); tr-=1; tc-=1
field = [input() for _ in range(H)]


dist = [[INF]*W for _ in range(H)]
dist[sr][sc] = 0

# bfs
q = deque([(sr, sc)])
while q:
    r, c = q.popleft()
    nd = dist[r][c] + 1
    for i, (dr, dc) in enumerate(AROUND):
        nr, nc = r+dr, c+dc
        while 0 <= nr < H and 0 <= nc < W and field[nr][nc] == "." and dist[nr][nc] >= nd:
            dist[nr][nc] = nd
            q.append((nr, nc))
            nr += dr
            nc += dc

    # print(*dist, sep="\n", end="\n\n")

print(dist[tr][tc]-1)
