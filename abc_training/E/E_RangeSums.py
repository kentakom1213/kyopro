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
    N, Q = map(int, input().split())

    uf = UnionFind(N+1)

    for _ in range(Q):
        l, r = map(int, input().split())
        l -= 1
        uf.union(l, r)
    
    print("Yes" if uf.same(0, N) else "No")
