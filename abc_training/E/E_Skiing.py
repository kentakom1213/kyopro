#              E - Skiing
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc237/tasks/abc237_e
# 解説
# https://www.youtube.com/watch?v=HOIUaAO-aWc

# AC
# ----------------------------------------

# 負の辺にはダイクストラは使えない
# 負の辺に使えるベルマンフォード法はO(NM) => TLE
# なんとかしてダイクストラが使える形に落とし込みたい

# 1. 登るときにかかるコストだけを重みにしたグラフを作成(下りは0)
# 2. 最短経路を探索 (ダイクストラ法)
# 3. 求める答えは、H_goal - H_start + cost の最大値になる


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

        for v, w_new in G[u]:
            if dist[v] > dist[u] + w_new:
                dist[v] = dist[u] + w_new  # 緩和処理
                heappush(q, (dist[v], v))
    
    return dist

if __name__ == "__main__":
    N, M = map(int, input().split())
    H = list(map(int, input().split()))
    G = [[] for _ in range(N)]
    for i in range(M):
        u, v = map(int, input().split())
        u-=1; v-=1

        h, l = (u, v) if H[u] > H[v] else (v, u)

        # descend
        G[h].append((l, 0))   # cost: 0
        # climb
        G[l].append((h, H[h] - H[l]))  # cost: diff

    # 最短経路探索
    cost = dijkstra(G, 0)
    
    # (スタートとの標高差 - コスト)の最大値が答えとなる
    ans = [H[0]-height-cost for height, cost in zip(H, cost)]
    # print(ans)

    print(max(ans))
