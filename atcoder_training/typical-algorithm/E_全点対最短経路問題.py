#              E - 全点対最短経路問題              
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/typical-algorithm/tasks/typical_algorithm_e
# ----------------------------------------

N, M = map(int, input().split())

G = [[1e20] * N for _ in range(N)]
for _ in range(M):
    u, v, c = map(int, input().split())
    G[u][v] = c

for i in range(N):
    G[i][i] = 0

# ワーシャル・フロイド法
for k in range(N):
    for i in range(N):
        for j in range(N):
            if G[i][j] > G[i][k] + G[k][j]:
                G[i][j] = G[i][k] + G[k][j]

ans = 0
for i in range(N):
    for j in range(N):
        ans += G[i][j]

print(ans)
