#       D - Shortest Path Queries 2       
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc208/tasks/abc208_d
# ----------------------------------------

N, M = map(int, input().split())

d = [[1<<60]*N for _ in range(N)]
for i in range(N):
    d[i][i] = 0

for i in range(M):
    a, b, c = map(int, input().split())
    d[a-1][b-1] = c

ans = 0
for k in range(N):
    nxt = [[0]*N for _ in range(N)]
    for s in range(N):
        for t in range(N):
            nxt[s][t] = min(d[s][t], d[s][k]+d[k][t])
            if nxt[s][t] < 1<<59:
                ans += nxt[s][t]
    d = nxt

print(ans)
