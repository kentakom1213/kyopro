#            D - Rectangles
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc218/tasks/abc218_d

# WA
# ----------------------------------------


from bisect import bisect_left as BS
def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

N = int(input())
points = [tuple(map(int, input().split())) for _ in range(N)]

# (x, y)それぞれに関して座標圧縮
vals_x = sorted(set(v[0] for v in points))
vals_y = sorted(set(v[1] for v in points))

c_points = [(BS(vals_x, x), BS(vals_y, y)) for x, y in points]

lx, ly = len(vals_x), len(vals_y)

DP = init_array(lx, ly)
for i in range(lx):
    for j in range(ly):
