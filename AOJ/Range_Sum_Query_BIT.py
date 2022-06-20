#             Range Sum Query             
# ----------------------------------------
# 問題
# https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_B
# ----------------------------------------

# BIT
class BIT:
    def __init__(self, n):
        self.size = n
        self.arr = [0] * (n + 1)
    
    def sum(self, i):
        s = 0
        while i:
            s += self.arr[i]
            i -= i & -i
        return s
    
    def add(self, i, x):
        while i <= self.size:
            self.arr[i] += x
            i += i & -i

def main():
    N, Q = map(int, input().split())

    bit = BIT(N)

    for _ in range(Q):
        com, x, y = map(int, input().split())
        if com == 0:
            bit.add(x, y)
        else:
            ans = bit.sum(y) - bit.sum(x-1)
            print(ans)

if __name__ == "__main__":
    main()
