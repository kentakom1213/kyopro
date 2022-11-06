#              E - Round Trip             
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc276/tasks/abc276_e
# ----------------------------------------

"""
## 参考
- [イカタコさん](https://atcoder.jp/contests/abc276/submissions/36239305)
"""

MOVE = [(-1, 0), (1, 0), (0, -1), (0, 1)]

def solve(h, w, field):
    sr, sc = 0, 0
    for i in range(H):
        for j in range(W):
            if field[i][j] == "S":
                sr, sc = i, j
                break

    visited = [[-1]*W for _ in range(H)]
    q = []
    for i, (dr, dc) in enumerate(MOVE):
        nr, nc = sr+dr, sc+dc
        if 0 <= nr < H \
        and 0 <= nc < W \
        and field[nr][nc] == ".":
            q.append((nr, nc))
            visited[nr][nc] = i

    while q:
        r, c = q.pop()
        for dr, dc in MOVE:
            nr, nc = r+dr, c+dc
            if not 0 <= nr < H \
            or not 0 <= nc < W \
            or field[nr][nc] != ".":
                continue
            if visited[nr][nc] == visited[r][c]:
                continue
            if visited[nr][nc] == -1:
                visited[nr][nc] = visited[r][c]
                q.append((nr, nc))
                continue
            return "Yes"
    return "No"

if __name__ == "__main__":
    H, W = map(int, input().split())
    C = [input() for _ in range(H)]
    print(solve(H, W, C))
