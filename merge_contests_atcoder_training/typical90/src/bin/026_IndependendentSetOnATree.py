#   026 - Independent Set on a Tree（★4）
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/typical90/tasks/typical90_z

# AC
# ----------------------------------------

# 解説
# https://kaage.hatenablog.com/entry/2021/05/04/144829

# 2部グラフという概念が必要らしい
# | 変で直接結ばれた頂点同士が互いに違う色になるように塗ることができるグラフ

# 木は必ず2部グラフになるから、depthが大きい順に出力すれば良い

# PyPy3   -> TLE
# Python3 -> AC

import sys
sys.setrecursionlimit(100000)

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

    dfs(0, 0)

    if sum( d & 1 for d in depth ) > N//2:
        print(*[i+1 for i, d in enumerate(depth) if d & 1][:N//2])
    else:
        print(*[i+1 for i, d in enumerate(depth) if not(d & 1)][:N//2])