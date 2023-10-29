#          D - LRUD Instructions          
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc273/tasks/abc273_d
# ----------------------------------------

# リファクタリング

from collections import defaultdict
from bisect import bisect_left, bisect_right

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

# 入力
Q = int(input())
for i in range(Q):
    d, l = input().split()
    l = int(l)

    if d == "L":
        lb = 0
        if r_walls[R]:
            C_idx = bisect_left(r_walls[R], C)
            if C_idx > 0:
                lb = r_walls[R][C_idx - 1]
        C = max(C-l, lb+1)
    elif d == "U":
        lb = 0
        if c_walls[C]:
            R_idx = bisect_left(c_walls[C], R)
            if R_idx > 0:
                lb = c_walls[C][R_idx - 1]
        R = max(R-l, lb+1)
    elif d == "R":
        ub = W+1
        if r_walls[R]:
            C_idx = bisect_right(r_walls[R], C)
            if C_idx < len(r_walls[R]):
                ub = r_walls[R][C_idx]
        C = min(C+l, ub-1)
    elif d == "D":
        ub = H+1
        if c_walls[C]:
            R_idx = bisect_right(c_walls[C], R)
            if R_idx < len(c_walls[C]):
                ub = c_walls[C][R_idx]
        R = min(R+l, ub-1)
    print(R, C)