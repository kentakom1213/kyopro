#      D - Number of Shortest paths
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc211/tasks/abc211_d
# ----------------------------------------

# bfs + dp

from collections import deque
MOD = 1000000007

N, M = map(int, input().split())
G = [[] for _ in range(N)]
for i in range(M):
    a, b = map(int, input().split())
    a -= 1; b -= 1
    G[a].append(b)
    G[b].append(a)

# bfs
path = [-1] * N
path[0] = 0
cnt = [0] * N
cnt[0] = 1
q = deque([0])
while q:

    cur = q.popleft()
    for nxt in G[cur]:
        if path[nxt] == -1:
            path[nxt] = path[cur] + 1
            cnt[nxt] = cnt[cur]
            q.append(nxt)
        elif path[nxt] == path[cur] + 1:
            cnt[nxt] += cnt[cur]
            cnt[nxt] %= MOD
    
print(cnt[N-1])
