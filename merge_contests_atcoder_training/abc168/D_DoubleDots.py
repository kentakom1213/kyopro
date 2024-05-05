#         D - .. (Double Dots)
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc168/tasks/abc168_d
# 解説
# https://motsu-xe.hatenablog.com/entry/2020/05/17/224220

# AC
# ----------------------------------------

from collections import deque

N, M = map(int, input().split())
G = [[] for _ in range(N)]
for _ in range(M):
    a, b = map(int, input().split())
    a-=1; b-=1
    G[a].append(b)
    G[b].append(a)

# bfs
parent = [-1] * N
stack = deque([0])
while stack:
    u = stack.popleft()
    for v in G[u]:
        if parent[v] != -1:
            continue
        parent[v] = u
        stack.append(v)

if -1 in parent:
    print("No")
else:
    print("Yes")
    for i in parent[1:]:
        print(i+1)
