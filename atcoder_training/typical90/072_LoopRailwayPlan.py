#       072 - Loop Railway Plan（★4）
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/typical90/tasks/typical90_bt
# ----------------------------------------

AROUND = [(1, 0), (0, 1), (-1, 0), (0, -1)]

H, W = map(int, input().split())
field = [input() for _ in range(H)]

# dfs
longest = []
def dfs(now, path=[]):
    global longest

    for dr, dc in AROUND:
        nxt = nr, nc = now[0]+dr, now[1]+dc

        if path and path[0] == nxt:
            if len(longest) < len(path):
                longest = path + [now]
            continue

        if 0 <= nr < H \
            and 0 <= nc < W \
            and field[nr][nc] == "." \
            and nxt not in path:
            dfs(nxt, path+[now])

# 全ての頂点について探索
for i in range(H):
    for j in range(W):
        if field[i][j] == ".":
            dfs((i, j))

if len(longest) < 4:
    print(-1)
else:
    print(len(longest))
