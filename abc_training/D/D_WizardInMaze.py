#           D - Wizard in Maze
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc176/tasks/abc176_d
# ----------------------------------------

# BBBBB
# BBABB
# BA*AB
# BBABB
# BBBBB

moveA = [(0, 1), (0, -1), (1, 0), (-1, 0)]
moveB = sum(([(dr, dc) for dc in range(-2, 3)] for dr in range(-2, 3)), [])

H, W = map(int, input().split())
ch, cw = map(int, input().split())
dh, dw = map(int, input().split())

S = [input() for _ in range(H)]

dist = [[0]*W for _ in range(H)]
warp = [[0]*W for _ in range(H)]

# 非再帰DFS
stack = [(ch, cw)]
while stack:
    pass
