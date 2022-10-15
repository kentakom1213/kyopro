#          D - LRUD Instructions          
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc273/tasks/abc273_d
# ----------------------------------------

from collections import defaultdict
from bisect import bisect_left

### SOLVE ###
H, W, R, C = map(int, input().split())
N = int(input())

r_walls = defaultdict(list)
c_walls = defaultdict(list)
for i in range(N):
    r, c = map(int, input().split())
    r_walls[r].append(c)
    c_walls[c].append(r)

for row in r_walls.values():
    row.sort()
for col in c_walls.values():
    col.sort()

# print(r_walls)
# print(c_walls)

# 入力
Q = int(input())
for i in range(Q):
    d, l = input().split()
    l = int(l)

    if d == "L":
        move_length = min(l, C-1)
        if r_walls[R]:
            C_idx = bisect_left(r_walls[R], C)
            if C_idx > 0:
                move_length = min(
                    move_length,
                    C - r_walls[R][C_idx - 1] - 1
                )
        C -= move_length
    elif d == "R":
        move_length = min(l, W-C)
        if r_walls[R]:
            C_idx = bisect_left(r_walls[R], C)
            if C_idx < len(r_walls[R]):
                move_length = min(
                    move_length,
                    r_walls[R][C_idx] - C - 1
                )
        C += move_length
    elif d == "U":
        move_length = min(l, R-1)
        if c_walls[C]:
            R_idx = bisect_left(c_walls[C], R)
            if R_idx > 0:
                move_length = min(
                    move_length,
                    R - c_walls[C][R_idx - 1] - 1
                )
        R -= move_length
    elif d == "D":
        move_length = min(l, H-R)
        if c_walls[C]:
            R_idx = bisect_left(c_walls[C], R)
            if R_idx < len(c_walls[C]):
                move_length = min(
                    move_length,
                    c_walls[C][R_idx] - R - 1
                )
        R += move_length

    print(R, C)