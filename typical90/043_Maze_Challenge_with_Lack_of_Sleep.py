# 043 - Maze Challenge with Lack of Sleep（★4）
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/typical90/tasks/typical90_aq
# ----------------------------------------

from heapq import heappush, heappop
INF = 1e20
AROUND = [(0, 1), (1, 0), (0, -1), (-1, 0)]  # [right, down, left, up]

H, W = map(int, input().split())
sr, sc = map(int, input().split()); sr-=1; sc-=1
tr, tc = map(int, input().split()); tr-=1; tc-=1
field = [input() for _ in range(H)]

# dijkstra
dist = [[INF]*W for _ in range(H)]
dist[sr][sc] = 0

pq = [(0, sr, sc, 0), (0, sr, sc, 1), (0, sr, sc, 2), (0, sr, sc, 3)]  # priority_queue
while pq:
    w, r, c, direct = heappop(pq)

    if dist[r][c] < w:
        continue

    for nxt_direct, (dr, dc) in enumerate(AROUND):
        nr, nc = r+dr, c+dc
        cost = 1 if direct != nxt_direct else 0
        if 0 <= nr < H and 0 <= nc < W and field[nr][nc] == ".":
            if dist[nr][nc] > dist[r][c] + cost:
                dist[nr][nc] = dist[r][c] + cost
                heappush(pq, (dist[nr][nc], nr, nc, nxt_direct))

print(dist[tr][tc])