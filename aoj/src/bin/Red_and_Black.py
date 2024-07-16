#              Red and Black              
# ----------------------------------------
# 問題
# https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1130&lang=jp
# ----------------------------------------

from collections import deque
INF = 1e20
MOVE = [(-1, 0), (1, 0), (0, -1), (0, 1)]

def solve():
    w, h = map(int, input().split())

    if w == h == 0:
        exit()
    
    F = [input() for _ in range(h)]

    # スタート位置の特定
    start = (0, 0)
    for r in range(h):
        for c in range(w):
            if F[r][c] == "@":
                start = (r, c)
    
    sr, sc = start

    # BFS
    visited = [[0] * w for _ in range(h)]
    visited[sr][sc] = 1
    que = deque([start])

    while que:
        cr, cc = que.popleft()
        for dr, dc in MOVE:
            nr = cr + dr
            nc = cc + dc
            if 0 <= nr < h \
                and 0 <= nc < w \
                and F[nr][nc] == "." \
                and visited[nr][nc] == 0:
                visited[nr][nc] = 1
                que.append((nr, nc))
    
    # 答え
    ans = 0
    for r in range(h):
        for c in range(w):
            ans += visited[r][c]
    
    print(ans)


def main():
    while True:
        solve()

main()
