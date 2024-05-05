#             C - NewFolder(1)            
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc261/tasks/abc261_c
# ----------------------------------------

from collections import defaultdict

N = int(input())
ss = [input() for _ in range(N)]
d = defaultdict(int)

for s in ss:
    if d[s]:
        print(f"{s}({d[s]})")
    else:
        print(s)
    d[s] += 1
