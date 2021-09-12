#                 C - ID
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc113/tasks/abc113_c

# AC
# ----------------------------------------

from bisect import bisect_left

N, M = map(int, input().split())
PY = [tuple(map(int, input().split())) for _ in range(M)]

dict_PY = {p:[] for p, _ in PY}
for p, y in PY:
    dict_PY[p].append(y)

for p, y in dict_PY.items():
    dict_PY[p] = sorted(set(y))

for p, y in PY:
    comp_y = bisect_left(dict_PY[p], y) + 1
    print(f"{p:0>6}{comp_y:0>6}")