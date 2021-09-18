#           C - Squered Error
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc194/tasks/abc194_c
# ----------------------------------------

# 二乗和誤差に関する問題
# 方針
# 全て計算すると、 O(N^2) で間に合わない
# |Ai| <= 200 を利用

N = int(input())
A = list(map(int, input().split()))

counter = [A.count(i) for i in range(-200, 201)]  # O(N)
# 400 * 3 * 10^5 ~= 10^8 で通らないか
# 線形だからダメ？？

# O(1)
res = 0
# for i in range(1, 401):
#     for j in range(i):
#         error = (i - j) ** 2
#         res += error * counter[i] * counter[j]

print(res)