#        D - バスと避けられない運命
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc012/tasks/abc012_4
# ----------------------------------------

from itertools import product
INF = 1e20

def main():
    N, M = map(int, input().split())

    dist = [[INF]*N for _ in range(N)]
    for i in range(N):
        dist[i][i] = 0

    for i in range(M):
        a, b, t = map(int, input().split())
        a-=1; b-=1
        dist[a][b] = dist[b][a] = t

    # ワーシャルフロイド法
    for c, a, b in product(range(N), repeat=3):
        dist[a][b] = min(
            dist[a][b],              # 直接の経路
            dist[a][c] + dist[c][b]  # cを経由した経路
        )
    
    # 最大値の最小化
    ans = INF
    for to in dist:
        ans = min(ans, max(to))
    
    print(ans)

main()
