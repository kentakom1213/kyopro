
# 幅優先探索によるサイクル検出

# 入力受け取り
N, M = map(int, input().split())  # 頂点の数 / 辺の数
G = [[] for _ in range(N)]
for _ in range(M):
    a, b = map(int, input().split())
    a -= 1
    b -= 1
    G[a].append(b)
    G[b].append(a)

# スタート地点からの深さを保存
depth = [-1] * N
contain_cycle = False

# dfs
def dfs(cur, d):
    global contain_cycle

    for nxt in G[cur]:
        if depth[nxt] == -1:
            depth[nxt] = d+1
            dfs(nxt, d+1)
        elif depth[nxt] == d-1:
            continue
        else:
            contain_cycle = True

# 呼び出し
depth[0] = 0
dfs(0, 0)

print("Yes" if contain_cycle else "No")
