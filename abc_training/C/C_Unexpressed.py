#             C - Unexpressed 
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc193/tasks/abc193_c

# AC (解説)
# ----------------------------------------

# N <= 10^10
# 方針
# 全て素因数分解は絶対に間に合わない
# 10^10以下の数で a^b として表せる数の表を作っておき、Nの値でアクセスする

# どうやって作る？
# aの上限は (10^5)^2==10^10 であることから10^5、
# bの上限は 2^34 > 10^10 であることから34であることがわかる。
# 1 <= b <= 33 に対して、
#     a^b がNを超えない最大のaを二分探索で探す。
#     aが定まったらaをresultに加算する
# 計算量は O(log(N))

# -> 2^4 == 4^2 に思い至らず...

N = int(input())

# 流石に遅すぎる
# pows = []
# for a in range(int(1e5) + 1):  # (10^5) ^2 == 10^10
#     for b in range(34):  # 2^34 > 10^10
#         p = a ** b
#         if p > 1e10: break
#         pows.insert(bisect_left(pows, p), p)

def binary_search(condition, min, max):
    while max - min > 1:
        center = int((max + min) // 2)
        if condition(center):
            min = center
        else:
            max = center
    return min

can_express = set()
for b in range(2, 34):  # 2^34 > 10^10
    max_a = binary_search(lambda x: x**b <= N, 1, 1e5+1)
    can_express |= {a ** b for a in range(2, max_a + 1)}

print(N - len(can_express))
