#               F - 最小全域木問題
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/typical-algorithm/tasks/typical_algorithm_f?lang=ja
# ----------------------------------------

from heapq import heappush, heappop

# 入力の受け取り
N, M = map(int, input().split())
edges = [list(map(int, input().split())) for _ in range(M)]

# グラフの構築
G = [[] for _ in range(N)]
for (u, v, c) in edges:
    G[u].append((v, c))
    G[v].append((u, c))

# プリム法
# 現在選択している頂点から選択していない頂点に向かう辺集合
q = []
for v, c in G[0]:
    heappush(q, (c, v))

# 到達した頂点
visited = [False] * N
visited[0] = True

# 最小木を構成する辺の集合
mst_sum = 0

while q:
    c, u = heappop(q)
    if visited[u]:
        continue
    visited[u] = True
    mst_sum += c
    # uから出ている頂点をqに追加する
    for v, c in G[u]:
        if visited[v]:
            continue
        heappush(q, (c, v))

print(mst_sum)
