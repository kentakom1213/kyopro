#         012 - Red Painting（★4）
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/typical90/tasks/typical90_l

# いいねえ

# AC
# ----------------------------------------

# 一回ごとに深さ探索をすると、O(HW) * O(Q)
# 最大では 2000 * 2000 * 100000 = 4e11

# もしかしてUnionFind??


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

def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

def get_id(r, c):
    return r * W + c

AROUND = [(-1, 0), (0, -1), (1, 0), (0, 1)]

if __name__ == "__main__":
    H, W = map(int, input().split())
    Q = int(input())
    queries = [tuple(map(int, input().split())) for _ in range(Q)]

    pathes = UnionFind(H*W)
    field = init_array(H, W, 0)

    for q in queries:

        # 結合
        if q[0] == 1:
            r, c = map(lambda x: x-1, q[1:])

            field[r][c] = 1

            for dr, dc in AROUND:
                nr, nc = r + dr, c + dc
                if 0 <= nr < H and 0 <= nc < W and field[nr][nc]:
                    # つなぐ
                    pathes.union(get_id(r, c), get_id(nr, nc))
        
        # 探索
        else:
            ra, ca, rb, cb = map(lambda x: x-1, q[1:])

            print("Yes" if field[ra][ca] and pathes.same(get_id(ra, ca), get_id(rb, cb)) else "No")

        # テスト
        # print(*field, sep="\n")
        # print(pathes)
        # print()