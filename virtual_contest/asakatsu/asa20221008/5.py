# https://atcoder.jp/contests/abc157/tasks/abc157_e

"""
## 方針
- BITを使ったRSQに変換
- 集合をbitで管理（1 << (26+1) = 134217728）
"""

# [SegmentTree with Monoid](https://ikatakos.com/pot/programming_algorithm/data_structure/segment_tree)
class SegmentTreeInjectable:
    def __init__(self, n, e_factory, operator):
        """
        :param n: 要素数
        :param e_factory: func() -> S 単位元を生成する関数
        :param operator: func(S, S) -> S 親ノードが子ノード同士を合成する関数
        """
        n2 = 1 << (n - 1).bit_length()
        self.offset = n2
        self.data = [e_factory() for _ in range(n2 << 1)]
        self.op = operator
        self.e = e_factory
 
    @classmethod
    def from_array(cls, arr, e_factory, operator):
        """ 既存の配列から生成 """
        ins = cls(len(arr), e_factory, operator)
        ins.data[ins.offset:ins.offset + len(arr)] = arr
        for i in range(ins.offset - 1, 0, -1):
            lch = i << 1
            ins.data[i] = operator(ins.data[lch], ins.data[lch + 1])
        return ins
 
    def update(self, i, x):
        """ Aiをxに上書き更新 """
        # 上書きでなくて加算などで更新したい場合は、get_point() で現在値を取得して呼び出し元で行う
        data = self.data
        op = self.op
        i += self.offset
        data[i] = x
        while i > 1:
            i >>= 1
            lch = i << 1
            data[i] = op(data[lch], data[lch + 1])
 
    def get_point(self, p):
        return self.data[p + self.offset]
 
    def get_range(self, l, r):
        """ [l, r) の値を得る """
        data = self.data
        op = self.op
        result_l = self.e()
        result_r = self.e()
 
        l += self.offset
        r += self.offset
        while l < r:
            if l & 1:
                result_l = op(result_l, data[l])
                l += 1
            if r & 1:
                r -= 1
                result_r = op(data[r], result_r)
            l >>= 1
            r >>= 1
 
        return op(result_l, result_r)

def get_char_val(c):
    return ord(c) - ord('a')

# solve
N = int(input())
S = input()

seg = SegmentTreeInjectable(N, lambda: 0, lambda x, y: x|y)

# 文字列を数字に変換しつつBITに保存
for i, c in enumerate(S):
    c_val = get_char_val(c)
    seg.update(i, 1 << c_val)

# # クエリ処理
Q = int(input())
for i in range(Q):
    q, x, y = input().split()
    if q == '1':
        idx = int(x) - 1
        y_val = 1 << get_char_val(y)

        # セグ木の更新
        seg.update(idx, y_val)

    if q == '2':
        l = int(x) - 1
        r = int(y)
        range_val = seg.get_range(l, r)
        # 1であるビットを数える
        ans = 0
        for p in range(30):
            ans += (range_val >> p) & 1
        print(ans)