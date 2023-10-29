#           E - Range Sums
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc238/tasks/abc238_e
# ----------------------------------------

# 累積和を取ることを考える
# S[j+1] - S[i] = sum(A[n] for n in range(i, j+1))

# S[N] - S[0] =   (S[N] - S[l1])
#               + (S[l1] - S[r1])
#               + (S[r1] - S[l2])
#               + ... + (S[rn] - S[0])
# (l, rはどっちでも良い)

# すなわち、l<->r という辺を持つ無向グラフに対して、0からNまでの経路が存在すればよい

import sys
sys.setrecursionlimit(1000000)

N, Q = map(int, input().split())
G = [[] for _ in range(N+1)]
for _ in range(Q):
    l, r = map(int, input().split())
    l -= 1
    G[l].append(r)
    G[r].append(l)

is_visited = [0] * (N+1)
def dfs(cur):
    is_visited[cur] = 1
    for nxt in G[cur]:
        if is_visited[nxt]:
            continue
        dfs(nxt)

dfs(0)
print("Yes" if is_visited[N] else "No")
