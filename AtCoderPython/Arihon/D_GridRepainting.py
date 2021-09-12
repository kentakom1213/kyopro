#            D - Grid Repainting
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc088/tasks/abc088_d

# 参考
# https://drken1215.hatenablog.com/entry/2019/03/09/154800
# ----------------------------------------

from collections import deque
INF = int(1e10)

# input
H, W = map(int, input().split())
field = [input() for _ in range(H)]

start = (0, 0)
goal = (H - 1, W - 1)

# bfs
def get_shortest_path():
    que = deque()
    que.append(start)
    D = [[INF]*W for _ in range(H)]
    D[0][0] = 0

    # search
    while que:
        y, x = que.popleft()
        if (y, x) == goal:
            break

        # look around
        directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]
        for dy, dx in directions:
            ny, nx = y + dy, x + dx

            if 0 <= ny < H and 0 <= nx < W and field[ny][nx] != "#" and D[ny][nx] == INF:
                que.append((ny, nx))
                D[ny][nx] = D[y][x] + 1

    shortest = D[goal[0]][goal[1]]
    return shortest if shortest != INF else False

# exection
shortest = get_shortest_path()

if shortest:
    white = sum(row.count(".") for row in field)
    print(white - get_shortest_path() - 1)
else:
    print(-1)
