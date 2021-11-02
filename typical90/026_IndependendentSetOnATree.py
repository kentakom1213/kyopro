#   026 - Independent Set on a Tree（★4）
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/typical90/tasks/typical90_z
# ----------------------------------------

# 解説
# https://kaage.hatenablog.com/entry/2021/05/04/144829

# 2部グラフという概念が必要らしい
# | 変で直接結ばれた頂点同士が互いに違う色になるように塗ることができるグラフ

# 木は必ず2部グラフになるから、depthが大きい順に出力すれば良い

def dfs(now, now_depth):
    if depth[now] != -1:
        return
    
    depth[now] = now_depth
    for next in G[now]:
        dfs(next, now_depth + 1)


if __name__ == "__main__":
    N = int(input())
    G = [[] for _ in range(N)]
    depth = [-1] * N

    for _ in range(N-1):
        a, b = map(int, input().split())
        a -= 1
        b -= 1
        G[a].append(b)
        G[b].append(a)

    print(depth)

    dfs(0, 0)
    print(depth)