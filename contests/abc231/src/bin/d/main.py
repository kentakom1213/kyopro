
# 辺が3つ以上存在しない
# 巡回路が存在しない
from collections import defaultdict

# unionfind
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


def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

if __name__ == "__main__":
    
    N, M = map(int, input().split())

    G = [[] for _ in range(N)]
    uf = UnionFind(N)

    isOK = True

    # グラフ作成 & 巡回路検出
    for i in range(M):
        a, b = map(int, input().split())
        a -= 1
        b -= 1
        G[a].append(b)
        G[b].append(a)

        if uf.same(a, b):
            isOK = False
        else:
            uf.union(a, b)

    for i, v in enumerate(G):
        isOK &= len(v) < 3


    print("Yes" if isOK else "No")