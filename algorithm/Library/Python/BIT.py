### BIT ###
class BIT:
    def __init__(self, N):
        self.size = N
        self.arr = [0] * (N+1)

    def add(self, i, x):
        while i <= self.size:
            self.arr[i] += x
            i += i & -i

    def sum(self, i):
        res = 0
        while i:
            res += self.arr[i]
            i -= i & -i
        return res

### テスト ###
# https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_B&lang=jp
if __name__ == "__main__":
    N, Q = map(int, input().split())
    bit = BIT(N)

    for i in range(Q):
        com, x, y = map(int, input().split())
        if com == 0:
            bit.add(x, y)
        elif com == 1:
            print(bit.sum(y) - bit.sum(x-1))
