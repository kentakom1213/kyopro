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


## solve
N = int(input())
ST = []
idx = {}

for i in range(N):
    s, t = input().split()
    ST.append((s,t))
    
    if s not in idx:
        idx[s] = len(idx)
    
    if t not in idx:
        idx[t] = len(idx)

# ノード数
V = len(idx)

## DAGであるかどうかを判定
isOK = True
uf = UnionFind(V)

for s, t in ST:
    si = idx[s]
    ti = idx[t]
    isOK &= not uf.same(si, ti)
    uf.union(si, ti)

print("Yes" if isOK else "No")
