#              C - Green Bin              
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc137/tasks/abc137_c
# ----------------------------------------

from collections import defaultdict

N = int(input())
dd = defaultdict(int)

for _ in range(N):
    s = "".join(sorted(input()))
    dd[s] += 1

ans = 0
for k, v in dd.items():
    ans += v * (v-1) // 2

print(ans)
