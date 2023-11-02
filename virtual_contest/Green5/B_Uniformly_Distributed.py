# https://atcoder.jp/contests/arc120/tasks/arc120_b

from collections import defaultdict
MOD = 998244353

H, W = map(int, input().split())
FIELD = [input() for _ in range(H)]

def can_reach(r, c):
    return 0 <= r < H and 0 <= c < W

# 斜めに取得する
naname = [defaultdict(int) for _ in range(H+W-1)]
ans = 1

for i in range(H+W-1):
    for x in range(H+W):
        r = i - x
        c = x
        if can_reach(r, c):
            naname[i][FIELD[r][c]] += 1

    if "R" in naname[i] and "B" in naname[i]:
        ans = 0
    
    if "." in naname[i] and len(naname[i]) == 1:
        ans = (ans * 2) % MOD

print(ans)