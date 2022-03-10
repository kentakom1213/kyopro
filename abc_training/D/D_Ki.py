#                 D - Ki
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc138/tasks/abc138_d
# ----------------------------------------

N, Q = map(int, input().split())
G = [[] for _ in range(N)]
for _ in range(N-1):
    a, b = map(int, input().split())
    a, b = a-1, b-1
    G[a].append(b)
    G[b].append(a)

queries = [tuple(map(int, input().split())) for _ in range(N)]


