#                 D - Ki
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc138/tasks/abc138_d
# ----------------------------------------

import sys
sys.setrecursionlimit(1000000)
from collections import defaultdict

N, Q = map(int, input().split())
G = [[] for _ in range(N)]
for _ in range(N-1):
    a, b = map(int, input().split())
    a, b = a-1, b-1
    G[a].append(b)
    G[b].append(a)

queries = defaultdict(int)
for _ in range(Q):
    p, x = map(int, input().split())
    queries[p-1] += x

is_visited = [0] * N
ans = [0] * N

# dfs
def dfs(cur, val):
    is_visited[cur] = 1
    val += queries[cur]
    ans[cur] = val
    for nxt in G[cur]:
        if is_visited[nxt]:
            continue
        dfs(nxt, val)

dfs(0, 0)
print(*ans, sep=" ")
