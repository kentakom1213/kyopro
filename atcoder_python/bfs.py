#           breadth first search
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc007/tasks/abc007_3

# 参考
# https://ebisuke33.hatenablog.com/entry/antbook-abc007c
# ----------------------------------------

from collections import deque

# functions
INF = int(1e9)
def exprint(l): return print(*l, sep="\n")


# input
R, C = map(int, input().split())
sy, sx = map(int, input().split())
gy, gx = map(int, input().split())

sy -= 1
sx -= 1
gy -= 1
gx -= 1

field = [input() for _ in range(R)]

directions = [(1, 0), (0, 1), (-1, 0), (0, -1)]

# bfs
def bfs():
    que = deque()
    que.append((sy, sx))
    D = [[INF]*C for _ in range(R)]
    D[sy][sx] = 0

    # search
    while que:
        y, x = que.popleft()

        if (y, x) == (gy, gx):
            break

        for dy, dx in directions:
            ny, nx = y + dy, x + dx

            if all((0 <= ny < R,
                    0 <= nx < C,
                    field[ny][nx] != "#",
                    D[ny][nx] == INF)):

                que.append((ny, nx))
                D[ny][nx] = D[y][x] + 1

                # print([ny, nx])  # test
                # exprint(D) # test
                # print(que) # test

    return D[gy][gx]


print(bfs())
