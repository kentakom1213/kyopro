#                 B - 道路工事                
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc032/tasks/arc032_2
# ----------------------------------------

class UnionFind:
    def __init__(self, N):
        self.N = N
        self.parents = [-1] * N
    
    def find(self, x):
        if self.parents[x] < 0:
            return x
        else:
            self.parents[x] = self.find(self.parents[x])  # 経路圧縮
            return self.parents[x]
    
    def same(self, x, y):
        return self.find(x) == self.find(y)

    def union(self, x, y):
        x = self.find(x)
        y = self.find(y)

        if x == y:
            return
        
        if self.parents[x] > self.parents[y]:
            x, y = y, x
        
        self.parents[x] += self.parents[y]
        self.parents[y] = x


N, M = map(int, input().split())

uf = UnionFind(N)

for _ in range(M):
    a, b = map(int, input().split())
    a-=1; b-=1
    uf.union(a, b)

# グループの数をカウント
cnt = set()
for i in range(N):
    cnt.add(uf.find(i))

print(len(cnt) - 1)
