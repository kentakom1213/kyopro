#          D - Even Relation
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc126/tasks/abc126_d
# ----------------------------------------

N = int(input())

G = [[] for _ in range(N)]
for _ in range(N-1):
    u, v, w = map(int, input().split())
    u-=1; v-=1
    G[u].append((v, w))
    G[v].append((u, w))

# DFS
stack = [(0, 0)]
color = [-1] * N
while stack:
    cur, d = stack.pop()
    color[cur] = d & 1
    for nxt, w in G[cur]:
        if color[nxt] != -1:
            continue
        stack.append((nxt, d+w))

if not any(color):
    color[0] = 1

print(*color, sep="\n")
