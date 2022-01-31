#       Single Source Shortest Path
# ----------------------------------------
# 問題
# https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_A&lang=ja

# AC
# ----------------------------------------

from heapq import heappop, heappush
INF = 1e10

def dijkstra(G, s):
    N = len(G)
    dist = [INF] * N
    dist[s] = 0  # スタートの初期化

    # ヒープ
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

if __name__ == "__main__":
    V, E, r = map(int, input().split())
    G = [[] for _ in range(V)]
    for i in range(E):
        s, t, d = map(int, input().split())
        G[s].append((d, t))  # 重み, 隣接頂点
    
    ans = dijkstra(G, r)
    for p in ans:
        print(p if p!=INF else "INF")
