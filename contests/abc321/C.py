# from scipy.special import comb
from itertools import combinations

# ↓数の見積もり、愚直に計算して良さそう
# N = 10
# ans = 0

# for i in range(N):
#     for j in range(i):
#         ans += comb(9, j)

# print(ans)

K = int(input())

DIGITS = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
res = []

for i in range(1, 11):
    for cmb in combinations(DIGITS, i):
        num = int("".join(map(str, cmb)))
        res.append(num)

res.sort()

print(res[K])
