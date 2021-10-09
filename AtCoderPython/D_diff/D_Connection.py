#                D - 連結
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc049/tasks/arc065_b

# 難しい
# よって、各都市を(道路に関する Union-Find 上での根の番号, 鉄道に関する Union-Find 上での根の番号)
# というペア値によって分類することで解ける。
# 計算量は、上に述べた「各都市をペア値に分類」という部分をstd::map
# によって実現した場合、O(NlogN+(K+L)α(N)) となる。

# AC (解説)
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
    N, K, L = map(int, input().split())

    road, train = UnionFind(N), UnionFind(N)

    for _ in range(K):
        p, q = map(int, input().split())
        road.union(p-1, q-1)
        
    for _ in range(L):
        r, s = map(int, input().split())
        train.union(r-1, s-1)
    
    # これ遅い
    # for i in range(N):
    #     connect = set(road.members(i)) & set(train.members(i))
    #     print(len(connect), end=" ")

    nums = {}
    for i in range(N):
        couple = road.find(i), train.find(i)

        if couple in nums:
            nums[couple] += 1
        else:
            nums[couple] = 1

    # 出力
    for i in range(N):
        couple = road.find(i), train.find(i)
        print(nums[couple], end=" ")
    print()