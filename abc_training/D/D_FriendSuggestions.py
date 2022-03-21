#         D - Friend Suggestions
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc157/tasks/abc157_d
# ----------------------------------------

"""comment
## 方針
- UnionFindでグループの大きさを取得
- あらかじめ、友達/ブロックの数を計算
- グループの大きさ - (友達 + ブロック) - 1
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


# solve
N, M, K = map(int, input().split())

friend = [0] * N
block = [set() for _ in range(N)]

uf = UnionFind(N)

for _ in range(M):
    a, b = map(int, input().split())
    a-=1; b-=1
    friend[a] += 1
    friend[b] += 1
    uf.union(a, b)

for _ in range(K):
    c, d = map(int, input().split())
    c-=1; d-=1
    block[c].add(d)
    block[d].add(c)

for i in range(N):
    suggest = set(uf.members(i)) - block[i]
    ans = len(suggest) - friend[i] - 1
    print(ans)
