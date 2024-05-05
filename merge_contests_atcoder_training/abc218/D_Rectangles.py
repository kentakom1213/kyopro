#            D - Rectangles
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc218/tasks/abc218_d

# WA
# ----------------------------------------


from bisect import bisect_left as BS
from operator import itemgetter
def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

N = int(input())
points = [tuple(map(int, input().split())) for _ in range(N)]

# (x, y)それぞれに関して座標圧縮
vals_x = sorted(set(v[0] for v in points))
vals_y = sorted(set(v[1] for v in points))

c_points = sorted( sorted( [(BS(vals_x, x), BS(vals_y, y)) for x, y in points] , key=itemgetter(1)), key=itemgetter(0) )
# c_points = [(BS(vals_x, x), BS(vals_y, y)) for x, y in points]

# print(c_points)
# DP = init_array(lx, ly)  # -> DPに翻弄されすぎ
# for i in range(lx):
#     for j in range(ly):
#         pass


############# 解説
# 左下と右上を定めると長方形が一意に定まる

### 位置
# UpperLeft  UpperRight
# LowerLeft  LowerRight

counter = 0
for i in range(N):
    for j in range(i+1, N):
        LL = c_points[i]
        UR = c_points[j]

        if LL[0] < UR[0] and LL[1] < UR[1]:
            UL = LL[0], UR[1]
            LR = UR[0], LL[1]

            if (UL in c_points) and (LR in c_points):
                # print()
                # print(UL, UR)
                # print(LL, LR)
                counter += 1

print(counter)

# -> TLE