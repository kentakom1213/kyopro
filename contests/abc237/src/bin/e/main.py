import heapq

def dijkstra(edges, num_node):
    """ 経路の表現
            [終点, 辺の値]
            A, B, C, D, ... → 0, 1, 2, ...とする """
    node = [float('inf')] * num_node    #スタート地点以外の値は∞で初期化
    node[0] = 0     #スタートは0で初期化

    node_name = []
    heapq.heappush(node_name, [0, 0])

    while len(node_name) > 0:
        #ヒープから取り出し
        _, min_point = heapq.heappop(node_name)

        #経路の要素を各変数に格納することで，視覚的に見やすくする
        for factor in edges[min_point]:
            goal = factor[0]   #終点
            cost  = factor[1]   #コスト

            #更新条件
            if node[min_point] + cost < node[goal]:
                node[goal] = node[min_point] + cost     #更新
                #ヒープに登録
                heapq.heappush(node_name, [node[min_point] + cost, goal])

    return node

if __name__ == "__main__":
    N, M = map(int, input().split())
    H = list(map(int, input().split()))
    G = [[] for _ in range(M)]
    for i in range(M):
        u, v = map(int, input().split())
        u -= 1
        v -= 1
        u_v = -(H[u] - H[v]) if H[u] > H[v] else -2 * (H[u] - H[v])
        v_u = -(H[v] - H[u]) if H[v] > H[u] else -2 * (H[v] - H[u])
        G[u].append([v, u_v])
        G[v].append([u, v_u])

    opt = dijkstra(G, N)

    print(-min(opt))
    