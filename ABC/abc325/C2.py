from collections import deque

MOVE = []
for i in range(-1, 2):
    for j in range(-1, 2):
        if i == j == 0:
            continue
        MOVE.append((i, j))

H, W = map(int, input().split())
S = [input() for _ in range(H)]

field = [[0] * W for _ in range(H)]
cnt = 0

for r in range(H):
    for c in range(W):

        if S[r][c] == "#" and field[r][c] == 0:
            # bfs
            q = deque([(r, c)])
            cnt += 1

            while q:
                r, c = q.pop()

                # 色を変える
                field[r][c] = cnt

                for dr, dc in MOVE:
                    nr = r + dr
                    nc = c + dc
                    if 0 <= nr < H and 0 <= nc < W and S[nr][nc] == "#" and field[nr][nc] == 0:
                        field[nr][nc] = cnt
                        q.append((nr, nc))


print(cnt)
