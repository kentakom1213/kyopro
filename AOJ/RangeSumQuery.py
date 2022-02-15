#        Range Sum Query (RSQ)
# ----------------------------------------
# 問題
# https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_B&lang=jp

# AC
# ----------------------------------------

class SegTree:
    def __init__(self, n, arr=None):
        self.seg_len = self.get_seglen(n)
        self.arr = [0] * self.seg_len * 2
        
        if arr:
            for i, v in enumerate(arr):
                self.add(i, v)  

    def add(self, i, val):
        i += self.seg_len
        self.arr[i] += val
        while True:
            i >>= 1
            if i == 0:
                break
            self.arr[i] = self.arr[i*2] + self.arr[i*2+1]

    def sum(self, l, r):
        l += self.seg_len
        r += self.seg_len
        ans = 0
        while l < r:
            if l & 1:
                ans += self.arr[l]
                l += 1
            l >>= 1
            if r & 1:
                ans += self.arr[r-1]
                r -= 1
            r >>= 1
        return ans

    @staticmethod
    def get_seglen(n):
        log2_n = 0
        while n:
            log2_n += 1
            n >>= 1
        return 1 << log2_n


if __name__ == "__main__":
    n, q = map(int, input().split())
    A = SegTree(n)
    for _ in range(q):
        c, x, y = map(int, input().split())
        if c == 0:
            A.add(x, y)
        else:
            print(A.sum(x, y+1))
