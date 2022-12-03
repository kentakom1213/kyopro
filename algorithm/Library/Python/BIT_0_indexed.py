### BIT ###
class BIT:
    """ 0-indexedインターフェース のBIT """
    def __init__(self, N):
        self.size = N
        self.arr = [0] * (N+1)

    def add(self, i, x):
        """ i(0-indexed) に x を加算 """
        i += 1  # 1-indexed に変換
        while i <= self.size:
            self.arr[i] += x
            i += i & -i

    def sum(self, i):
        """ 区間 [0,i) の総和を返す """
        res = 0
        while i:
            res += self.arr[i]
            i -= i & -i
        return res
    
    def sum_range(self, l, r):
        """ 区間 [l,r) の総和を返す """
        to_l = self.sum(l)
        to_r = self.sum(r)
        return to_r - to_l

### テスト ###
# https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_B&lang=jp
if __name__ == "__main__":
    N, Q = map(int, input().split())
    bit = BIT(N)

    for i in range(Q):
        com, x, y = map(int, input().split())
        if com == 0:
            bit.add(x-1, y)
        elif com == 1:
            print(bit.sum_range(x-1, y))
        print(bit.arr)
