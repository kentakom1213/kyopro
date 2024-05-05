from collections import deque

N, M = map(int, input().split())
S = [input() for _ in range(N)]

# 進む方向
MOVE = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
]

# 最終的に触れることができる地点
is_ok = [[False] * M for _ in range(N)]
is_ok[1][1] = True

# 今進んでいる向きで到達できるかどうか
dp = [[[False] * 4 for _ in range(M)] for _ in range(N)]
dp[1][1][0] = True
dp[1][1][1] = True
dp[1][1][2] = True
dp[1][1][3] = True

# q : (row, col, dir)
q = deque([(1, 1, 0), (1, 1, 1), (1, 1, 2), (1, 1, 3)])

while q:
    r, c, d = q.popleft()
    # 進む
    dr, dc = MOVE[d]
    nr = r + dr
    nc = c + dc

    # 進む方向に岩がある
    if S[nr][nc] == "#":
        # dを除いた方向に進む
        for (nd, (ddr, ddc)) in enumerate(MOVE):
            if d == nd:
                continue
            nnr = r + ddr
            nnc = c + ddc

            if S[nnr][nnc] == "." and not dp[nnr][nnc][nd]:
                is_ok[nnr][nnc] = True
                dp[nnr][nnc][nd] = True
                q.append((nnr, nnc, nd))
    # 進む方向が氷
    else:
        # dの方向に進む
        is_ok[nr][nc] = True
        dp[nr][nc][d] = True
        q.append((nr, nc, d))


# 数える
ans = 0
for row in is_ok:
    ans += sum(row)

print(ans)
