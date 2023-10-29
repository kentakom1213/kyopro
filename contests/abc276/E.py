import sys
sys.setrecursionlimit(1000000)

MOVE_L = [(-1, 0), (0, -1), (1, 0), (0, 1)]
MOVE_R = [(0, 1), (1, 0), (0, -1), (-1, 0)]

ans = {}

def dfs_l(r, c, route):
    global ans
    for dr, dc in MOVE_L:
        nr = r + dr
        nc = c + dc
        if 0 <= nr < H and 0 <= nc < W and field[nr][nc] != "#":
            if (nr, nc) in route and route[(nr, nc)] == 0:
                if len(ans) < len(route):
                    ans = route.copy()
            elif (nr, nc) not in route:
                route[(nr, nc)] = len(route)
                dfs_l(nr, nc, route)
                del route[(nr, nc)]

if __name__ == "__main__":
    
    H, W = map(int, input().split())
    field = [input() for _ in range(H)]

    # スタート
    sr = sc = 0
    for i in range(H):
        for j in range(W):
            if field[i][j] == 'S':
                sr, sc = i, j

    dfs_l(sr, sc, {(sr, sc): 0})

    if len(ans) >= 4:
        print("Yes")
    else:
        print("No")