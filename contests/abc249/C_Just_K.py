#                C - Just K               
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc249/tasks/abc249_c
# ----------------------------------------

from collections import Counter

N, K = map(int, input().split())
SS = [input() for _ in range(N)]

ans = 0

# bit全探索
for i in range(1 << N):
    ss = ""
    for j in range(N):
        if (i>>j) & 1:
            ss += SS[j]

    cnt = Counter(ss)

    ok = 0
    for k, v in cnt.items():
        ok += v == K
    
    ans = max(ans, ok)

print(ans)
