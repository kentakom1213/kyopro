#           F - Range Xor Query           
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc185/tasks/abc185_f
# ----------------------------------------

import sys
input = sys.stdin.readline

# BIT
class BIT:
    """ 0-indexedインターフェース のBIT """
    def __init__(self, N, e_factory, f, inv):
        self.size = N
        self.arr = [e_factory()] * (N+1)
        self.e = e_factory
        self.f = f
        self.inv = inv

    def update(self, i, x):
        """ i(0-indexed) に x を加算 """
        i += 1  # 1-indexed に変換
        while i <= self.size:
            self.arr[i] = self.f(self.arr[i], x)
            i += i & -i

    def fold(self, i):
        """ 区間 [0,i) の総和を返す """
        res = 0
        while i:
            res = self.f(res, self.arr[i])
            i -= i & -i
        return res
    
    def get_range(self, l, r):
        """ 区間 [l,r) の総和を返す """
        to_l_inv = self.inv( self.fold(l) )
        to_r = self.fold(r)
        return self.f( to_l_inv, to_r )


# solve
N, Q = map(int, input().split())
A = list(map(int, input().split()))

bit = BIT(N, lambda: 0, lambda x,y: x^y, lambda x:x)

# セット
for i, a in enumerate(A):
    bit.update(i, a)

for i in range(Q):
    t, x, y = map(int, input().split())
    x -= 1

    if t == 1:
        bit.update(x, y)
    if t == 2:
        val = bit.get_range(x, y)
        print(val)