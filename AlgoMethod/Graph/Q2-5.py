# https://algo-method.com/tasks/434

import sys
sys.setrecursionlimit(1000000)

def dfs(cur, cnt):
    for nxt in G[cur]:
        dfs(nxt, cnt)
        cnt[cur] += cnt[nxt]

N = int(input())
P = list(map(int, input().split()))
G = [[] for _ in range(N)]
for i, p in enumerate(P):
    G[p].append(i+1)

cnt = [1] * N
dfs(0, cnt)

for c in cnt:
    print(c-1)
