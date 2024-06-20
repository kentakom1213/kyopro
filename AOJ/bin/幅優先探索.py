#                  幅優先探索                  
# ----------------------------------------
# 問題
# https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_11_C&lang=ja
# ----------------------------------------

from collections import deque

N = int(input())

G = []
for i in range(N):
    _, _, *adj = (int(v) - 1 for v in input().split())
    G.append(adj)

## 幅優先探索
que = deque([0]) # 未探索の頂点を管理するキュー
dist = [-1] * N
dist[0] = 0

while que:
    u = que.popleft()
    for v in G[u]:
        if dist[v] == -1:
            dist[v] = dist[u] + 1
            que.append(v)

# 出力
for i in range(N):
    print(i + 1, dist[i])
