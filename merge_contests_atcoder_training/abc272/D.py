
from collections import deque

SIGN = [(1, 1), (1, -1), (-1, 1), (-1, -1)]

def can_reach(r, c):
    return 0 <= r < N and 0 <= c < N

def bfs(start, m):
    que = deque()
    que.append(start)

    MOVE = []
    i = 0
    while i*i <= m:
        j = 0
        while i*i + j*j <= m:
            if i*i + j*j == m:
                MOVE.append((i, j))
            j += 1
        i += 1

    while que:
        cr, cc = que.popleft()
        for dr, dc in MOVE:
            for sr, sc in SIGN:
                nr = cr + dr * sr
                nc = cc + dc * sc
                if can_reach(nr, nc) and field[nr][nc] > field[cr][cc] + 1:
                    field[nr][nc] = field[cr][cc] + 1
                    que.append((nr, nc))
            

if __name__ == "__main__":
    N, M = map(int, input().split())

    field = [[1e20]*N for _ in range(N)]
    field[0][0] = 0

    # 探索
    bfs((0, 0), M)

    for row in field:
        for v in row:
            if v >= 1e18:
                print(-1, end=" ")
            else:
                print(v, end=" ")
        print()