#            Tolopogocal Sort
# ----------------------------------------
# 問題
# https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_4_B&lang=jp

# AC
# ----------------------------------------

from collections import deque

def topologocal_sort(G):
    N = len(G)
    in_cnt = [0] * N
    for edges in G:
        for to in edges:
            in_cnt[to] += 1
    res = []
    q = deque(i for i in range(N) if in_cnt[i] == 0)
    while q:
        u = q.popleft()
        res.append(u)
        for v in G[u]:
            in_cnt[v] -= 1
            if in_cnt[v] == 0:
                q.append(v)
    return res

if __name__ == "__main__":
    v, e = map(int, input().split())
    G = [[] for _ in range(v)]
    for _ in range(e):
        s, t = map(int, input().split())
        G[s].append(t)
    
    ans = topologocal_sort(G)
    print(*ans, sep="\n")
