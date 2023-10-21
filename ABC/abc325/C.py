from collections import deque

H, W = map(int, input().split())
S = [input() for _ in range(H)]

field = [[-1] * W for _ in range(H)]

def dfs(i, j, c):
    global field
    if S[i][j] == "." or field[i][j] != -1:
        return
    field[i][j] = c

    for di in range(-1, 2):
        for dj in range(-1, 2):
            ni = i + di
            nj = j + dj

            if (
                0 <= ni < H 
                and 0 <= nj < W
                and S[ni][nj] == "#"
                and field[ni][nj] == -1
            ):
                field[ni][nj] = c
                dfs(ni, nj, c)

# 色塗り
c = 0
for i in range(H):
    for j in range(W):

        dfs(i, j, c)
        c += 1


        for row in field:
            print(row)
        print()
