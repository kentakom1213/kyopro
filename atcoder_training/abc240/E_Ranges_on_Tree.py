#            E - Ranges on Tree           
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc240/tasks/abc240_e
# ----------------------------------------

import sys
sys.setrecursionlimit(1000000)

N = int(input())
edges = [tuple(int(v) - 1 for v in input().split()) for _ in range(N - 1)]

# グラフの構築
G = [[] for _ in range(N)]
for u, v in edges:
    G[u].append(v)
    G[v].append(u)

# 変数の定義
x = 1
left = [-1] * N
right = [-1] * N

def rec(p, u):
    global x, left, right
    if len(G[u]) == 1 and G[u][0] == p:
        left[u] = right[u] = x
        x += 1
        return
    
    l = x
    r = x
    for v in G[u]:
        if v == p:
            continue
        rec(u, v)
        l = min(l, left[v])
        r = max(r, right[v])
    left[u] = l
    right[u] = r

rec(-1, 0)

# 表示
for i in range(N):
    print(left[i], right[i])
