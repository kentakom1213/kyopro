#             E - Unique Color            
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc198/tasks/abc198_e
# ----------------------------------------

"""comment
## 方針
- 1からxへのパスは一通りに定まるため、dfsでも最短経路がもとまる
- 経路をdictで管理しつつdfs
"""

import sys
sys.setrecursionlimit(1000000)
from collections import defaultdict

# input
N = int(input())
C = list(map(int, input().split()))
G = [[] for _ in range(N)]
for _ in range(N-1):
    a, b = map(int, input().split())
    a-=1; b-=1
    G[a].append(b)
    G[b].append(a)


# solve
is_good = [-1] * N  # -1: 未到達, 0: not good, 1: good
path = defaultdict(int)

def dfs(cur, path):
    is_good[cur] = path[C[cur]] == 0

    path[C[cur]] += 1
    for nxt in G[cur]:
        if is_good[nxt] != -1:
            continue
        dfs(nxt, path)
    path[C[cur]] -= 1


dfs(0, path)

is_good[0] = 1
for i in range(N):
    if is_good[i]:
        print(i+1)
