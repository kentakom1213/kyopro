#              D - 単一始点最短経路問題             
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/typical-algorithm/tasks/typical_algorithm_d
# ----------------------------------------

from heapq import heappush, heappop

N, M = map(int, input().split())

# グラフの構築
G = [[] for _ in range(N)]
for _ in range(M):
    u, v, c = map(int, input().split())
    G[u].append((v, c))

# ダイクストラ法
pq = [(0, 0)] # (現在の最短経路, 頂点)
dist = [1e20] * N
dist[0] = 0

while pq:
    d, u = heappop(pq)
    if dist[u] < d:
        continue
    for v, c in G[u]:
        # 緩和処理
        if dist[v] > d + c:
            dist[v] = d + c
            heappush(pq, (dist[v], v))

print(dist)
print(dist[N - 1])
