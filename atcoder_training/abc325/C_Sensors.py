#               C - Sensors               
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc325/tasks/abc325_c
# ----------------------------------------

H, W = map(int, input().split())
S = [input() for _ in range(H)]

field = [[0] * W for _ in range(H)]

cnt = 0

for i in range(H):
    for j in range(W):
        if S[i][j] == "#" and field[i][j] == 0:
            cnt += 1
            field[i][j] = cnt

            # dfs
            st = [(i, j)]
            while st:
                r, c = st.pop()
                field[r][c] = cnt

                for dr in range(-1, 2):
                    for dc in range(-1, 2):
                        nr = r + dr
                        nc = c + dc
                        if (0 <= nr < H
                        and 0 <= nc < W
                        and S[nr][nc] == "#"
                        and field[nr][nc] == 0):
                            field[nr][nc] = cnt
                            st.append((nr, nc))

print(cnt)
