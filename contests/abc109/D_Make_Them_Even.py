#            D - Make Them Even           
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc109/tasks/abc109_d
# ----------------------------------------

"""
## 方針
- 貪欲に処理できる
- field[i][j]が奇数のときその前後左右どれかのますが奇数だったら、そちらへ移動
- 周りが全て奇数の場合は何もしない
"""

import sys
input = sys.stdin.readline

H, W = map(int, input().split())
field = [list(map(int, input().split())) for _ in range(H)]

def around_cells(i, j):
    res = []
    for dr, dc in [(-1, 0), (0, -1), (0, 1), (1, 0)]:
        nr, nc = i+dr, j+dc
        if 0 <= nr < H and 0 <= nc < W:
            res.append((nr, nc))
    return res

ans = []
for i in range(H):
    for j in range(W):
        if field[i][j] & 1:
            for nr, nc in around_cells(i, j):
                if field[nr][nc] & 1:
                    field[i][j] -= 1
                    field[nr][nc] += 1
                    ans.append((i, j, nr, nc))
                    break
            else:
                if j+1 < W:
                    field[i][j] -= 1
                    field[i][j+1] += 1
                    ans.append((i, j, i, j+1))
                elif i+1 < H:
                    field[i][j] -= 1
                    field[i+1][j] += 1
                    ans.append((i, j, i+1, j))

print(len(ans))
for i, j, nr, nc in ans:
    print(i+1, j+1, nr+1, nc+1)