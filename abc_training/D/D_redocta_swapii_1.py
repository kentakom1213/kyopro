#        D - "redocta".swap(i,i+1)        
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc264/tasks/abc264_d
# ----------------------------------------

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

def inversion_number(perm):
    sup = max(perm)
    bit = BIT(sup)
    res = 0
    for i, p in enumerate(perm):
        bit.add(p, 1)
        res += i + 1 - bit.sum(p)
    return res

# solve
atcoder_dict = {"atcoder"[i]:i+1 for i in range(7)}
order = lambda c : atcoder_dict[c]

S = input()

ans = inversion_number(list(map(order, S)))
print(ans)
