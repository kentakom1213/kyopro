from collections import deque
INF = 1e20
# トポソ

N, M = map(int, input().split())
G = [[] for _ in range(N)]
indegree = [0] * N # 入次数

for _ in range(M):
    x, y = map(int, input().split())
    x -= 1; y -= 1
    G[x].append(y)
    indegree[y] += 1

# 何回目に追加されたか
turn = [INF] * N

# 入次数0の頂点を集める
que = deque()
for i in range(N):
    if indegree[i] == 0:
        que.append(i)
        turn[i] = 0

# キューがからになるまで繰り返す
sorted_v = []
while que:
    u = que.popleft()
    sorted_v.append(u)

    for v in G[u]:
        indegree[v] -= 1
        if indegree[v] == 0:
            que.append(v)
            turn[v] = turn[u] + 1

# 実行不可能な時
if len(set(turn)) < N:
    print("No")
    exit()

# 表示
print("Yes")
for v in turn:
    print(v+1)
