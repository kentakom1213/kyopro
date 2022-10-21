#          E - Erasing Vertices 2         
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc267/tasks/abc267_e
# ----------------------------------------

from collections import deque

N, M = map(int, input().split())
A = list(map(int, input().split()))
G = [[] for _ in range(N)]
COST = [0] * N
for _ in range(M):
    u, v = map(int, input().split())
    u -= 1; v -= 1
    G[u].append(v)
    G[v].append(u)
    COST[u] += A[v]
    COST[v] += A[u]

def can(m):
    cost = COST[:]
    
    is_delete = [False] * N
    que = deque()
    for u in range(N):
        if cost[u] <= m:
            que.append(u)

    while que:
        u = que.popleft()
        for v in G[u]:
            cost[v] -= A[u]

            if not is_delete[v] and cost[v] <= m:
                is_delete[v] = True
                que.append(v)
    
    return all(is_delete)

l, r = -1, 1001001001001
while (r - l) > 1:
    mid = (l + r) // 2
    if can(mid):
        r = mid
    else:
        l = mid

print(r)