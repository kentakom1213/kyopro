
# 普通にbfsしよう

from collections import deque

H, W = map(int, input().split())
field = [input() for _ in range(H)]

directions = [(1, 0), (0, 1)]

is_visited = [[0] * W for _ in range(H)]

def bfs(start):
    q = deque()
    q.append(start)
    is_visited[start[0]][start[1]] = 1

    while q:
        now = q.popleft()
        r, c = now

        for dr, dc in directions:
            nr, nc = r+dr, c+dc

            if 0 <= nr < H \
                and 0 <= nc < W \
                and field[nr][nc] == "." \
                and is_visited[nr][nc] == 0:
                is_visited[nr][nc] = is_visited[r][c] + 1  # 探索済みに
                q.append((nr, nc))  # qに追加
    

bfs((0, 0))

print(max( sum(is_visited, []) ))