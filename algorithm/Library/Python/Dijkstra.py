from heapq import heappop, heappush
INF = 1e18

def dijkstra(G, s):
    N = len(G)
    dist = [INF] * N
    dist[s] = 0  # スタートの初期化

    q = [(0, s)]
    while q:
        w_old, u = heappop(q)

        if dist[u] < w_old:
            continue

        for w_new, v in G[u]:
            if dist[v] > dist[u] + w_new:
                dist[v] = dist[u] + w_new  # 緩和処理
                heappush(q, (dist[v], v))
    
    return dist