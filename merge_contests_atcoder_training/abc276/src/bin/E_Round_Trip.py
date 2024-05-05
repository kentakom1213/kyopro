#              E - Round Trip             
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc276/tasks/abc276_e
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

# solve
H, W = map(int, input().split())
C = [input() for _ in range(H)]

sr, sc = 0, 0
for i in range(H):
    for j in range(W):
        if C[i][j] == "S":
            sr, sc = i, j

# 連結判定
uf = UnionFind(H*W)
for i in range(H):
    for j in range(W):
        v = W*i+j
        if C[i][j] != ".":
            continue
        if i+1 < H and C[i+1][j] == ".":
            nv = W*(i+1)+j
            uf.union(v, nv)
        if j+1 < W and C[i][j+1] == ".":
            nv = W*i+j+1
            uf.union(v, nv)

adj = []
for dr, dc in [(-1, 0), (1, 0), (0, 1), (0, -1)]:
    nr, nc = sr+dr, sc+dc
    if 0 <= nr < H \
    and 0 <= nc < W \
    and C[nr][nc] == ".":
        adj.append((nr, nc))

ladj = len(adj)
for i in range(ladj):
    for j in range(i+1, ladj):
        a, b = adj[i], adj[j]

        if uf.same(W*a[0]+a[1], W*b[0]+b[1]):
            print("Yes")
            exit()

print("No")
