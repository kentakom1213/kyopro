#               E - Peddler               
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc188/tasks/abc188_e
# ----------------------------------------

"""comment
## 方針
- Xi < Yi であることから、DAG
- トポソしてから差の最大値を順に検索
"""

from collections import defaultdict

class UnionFind():
    def __init__(self, n):
        self.n = n
        self.parents = [-1] * n

    def find(self, x):
        if self.parents[x] < 0:
            return x
        else:
            self.parents[x] = self.find(self.parents[x])
            return self.parents[x]

    def union(self, x, y):
        x = self.find(x)
        y = self.find(y)

        if x == y:
            return

        if self.parents[x] > self.parents[y]:
            x, y = y, x

        self.parents[x] += self.parents[y]
        self.parents[y] = x

    def size(self, x):
        return -self.parents[self.find(x)]

    def same(self, x, y):
        return self.find(x) == self.find(y)

    def members(self, x):
        root = self.find(x)
        return [i for i in range(self.n) if self.find(i) == root]

    def roots(self):
        return [i for i, x in enumerate(self.parents) if x < 0]

    def group_count(self):
        return len(self.roots())

    def all_group_members(self):
        group_members = defaultdict(list)
        for member in range(self.n):
            group_members[self.find(member)].append(member)
        return group_members

    def __str__(self):
        return '\n'.join(f'{r}: {m}' for r, m in self.all_group_members().items())


def main():
    N, M = map(int, input().split())
    A = list(map(int, input().split()))

    G = [[] for _ in range(N)]
    uf = UnionFind(N)

    for _ in range(M):
        x, y = map(int, input().split())
        x-=1; y-=1
        G[x].append(y)
        uf.union(x, y)
    
    # トポロジカルソート
    in_degree = [0] * N  # 入次数
    for to in sum(G, []):
        in_degree[to] += 1
    
    q = []  # 入次数が0の頂点を保持
    for i, d in enumerate(in_degree):
        if d == 0:
            q.append(i)
    
    sorted_v = []  # ソート済み
    while q:
        v = q.pop()
        for u in G[v]:
            in_degree[u] -= 1
            if in_degree[u] == 0:
                q.append(u)
    
        sorted_v.append(v)
    
    # 差の最大値を検索
    mini_v, ans = sorted_v[0], -1e10
    for v in sorted_v:
        if uf.same(mini_v, v):
            if ans < A[v] - A[mini_v]:
                ans = A[v] - A[mini_v]
                
        if A[v] < A[mini_v]:
            mini_v = v

    print(ans)


if __name__ == "__main__":
    main()
