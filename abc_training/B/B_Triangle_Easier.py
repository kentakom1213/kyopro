#          B - Triangle (Easier)          
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc262/tasks/abc262_b
# ----------------------------------------

N, M = map(int, input().split())
G = [[0]*N for _ in range(N)]

for i in range(M):
    u, v = map(int, input().split())
    u-=1; v-=1
    G[u][v] = 1
    G[v][u] = 1

ans = 0
for a in range(N):
    for b in range(a+1, N):
        for c in range(b+1, N):
            ans += G[a][b] * G[b][c] * G[c][a]

print(ans)
