#           D - Wizard in Maze
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc176/tasks/abc176_d

# AC
# ----------------------------------------

# BBBBB
# BBABB
# BA*AB
# BBABB
# BBBBB

from collections import deque

moveA = [(0, 1), (0, -1), (1, 0), (-1, 0)]

H, W = map(int, input().split())
ch, cw = [int(v)-1 for v in input().split()]
dh, dw = [int(v)-1 for v in input().split()]

S = [input() for _ in range(H)]

dist = [[-1]*W for _ in range(H)]

dist[ch][cw] = 0

que = deque()
que.append((ch, cw))
warp = deque()

while que or warp:
    # 移動A
    while que:
        nr, nc = que.popleft()
        warp.append((nr, nc, dist[nr][nc]))
        for dr, dc in moveA:
            nxr, nxc = nr+dr, nc+dc
            if 0 <= nxr < H \
                and 0 <= nxc < W \
                and S[nxr][nxc] == "." \
                and dist[nxr][nxc] == -1:
                dist[nxr][nxc] = dist[nr][nc]
                que.append((nxr, nxc))
                warp.append((nxr, nxc, dist[nr][nc]))
    # 移動B
    while warp:
        nr, nc, cnt = warp.popleft()
        for dr in range(-2, 3):
            for dc in range(-2, 3):
                # 移動しない/移動Aの場合は無視
                if (dr, dc) == (0, 0) or (dr, dc) in moveA: continue
                # ワープ移動
                nxr, nxc = nr+dr, nc+dc
                if 0 <= nxr < H \
                    and 0 <= nxc < W \
                    and S[nxr][nxc] == "." \
                    and dist[nxr][nxc] == -1:
                    dist[nxr][nxc] = cnt + 1  # ワープ回数を加算
                    que.append((nxr, nxc))

print(dist[dh][dw])
