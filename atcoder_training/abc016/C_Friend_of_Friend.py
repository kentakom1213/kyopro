# C - 友達の友達
# ---------------
# 問題
# https://atcoder.jp/contests/abc016/tasks/abc016_3
# ---------------

N, M = map(int, input().split())
G = [set() for _ in range(N)]

for i in range(M):
    a, b = (int(v)-1 for v in input().split())
    G[a].add(b)
    G[b].add(a)

# ループで全探索
for i in range(N):
    friend_and_me = {i, *G[i]}
    friend_of_friend = set()
    for j in G[i]:
        friend_of_friend |= G[j] - friend_and_me

    print(len(friend_of_friend))
