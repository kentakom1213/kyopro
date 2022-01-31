#              E - Skiing
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc237/tasks/abc237_e
# ----------------------------------------

# 負の辺にはダイクストラは使えない
# 負の辺に使えるベルマンフォード法はO(NM) => TLE
# なんとかしてダイクストラが使える形に落とし込みたい


if __name__ == "__main__":
    N, M = map(int, input().split())
    H = list(map(int, input().split()))
    G = [[] for _ in range(N)]
    for i in range(M):
        u, v = map(int, input().split())
        u-=1; v-=1
        G[u].append(v)
        G[v].append(u)
