#      D - Number of Shortest paths
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc211/tasks/abc211_d
# ----------------------------------------

N, M = map(int, input().split())
G = [[] for _ in range(N)]
for i in range(M):
    a, b = map(int, input().split())
    a -= 1; b -= 1
    G[a].append(b)
    G[b].append(a)

def bfs(G, start):
    pass
