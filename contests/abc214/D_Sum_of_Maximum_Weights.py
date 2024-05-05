#       D - Sum of Maximum Weights
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc214/tasks/abc214_d

# 難しい
# AC
# ----------------------------------------

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


if __name__ == "__main__":
    N = int(input())
    edges = []
    for _ in range(N-1):
        u, v, w = map(int, input().split())
        edges.append((w, u-1, v-1))
    edges.sort()
    
    uf = UnionFind(N)

    ans = 0
    for w, u, v in edges:
        ans += w * uf.size(u) * uf.size(v)
        uf.union(u, v)
    
    print(ans)
