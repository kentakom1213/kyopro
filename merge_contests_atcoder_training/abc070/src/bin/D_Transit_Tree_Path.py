#          D - Transit Tree Path          
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc070/tasks/abc070_d
# ----------------------------------------

"""
## 考察
- $N \le 10^5, Q \le 10^5$ だから $O(NQ)$ は間に合わない
- 1つのクエリを $O(1)$ ないしは $O(log(Q))$ で解く必要がある

## 方針
- $K$ を根とする根付き木として扱う
- あらかじめ各頂点に関して $K$ からの距離を計算しておく
"""

import sys
input = sys.stdin.readline
sys.setrecursionlimit(1000000)

# input
N = int(input())
G = [[] for _ in range(N)]
for _ in range(N-1):
    a, b, c = map(int, input().split())
    a-=1; b-=1
    G[a].append((b, c))
    G[b].append((a, c))

Q, K = map(int, input().split())
K -= 1
queries = [[int(n)-1 for n in input().split()] for _ in range(Q)]

# 前計算
dist = [-1] * N
dist[K] = 0
def dfs(cur):
    for nxt, w in G[cur]:
        if dist[nxt] >= 0:
            continue
        dist[nxt] = dist[cur] + w
        dfs(nxt)

dfs(K)

# クエリ処理
for x, y in queries:
    d = dist[x] + dist[y]
    print(d)
