#      085 - Multiplication 085（★4）
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/typical90/tasks/typical90_cg
# ----------------------------------------

# aの最大値は 10**(1/3)

from math import ceil

K = int(input())

res = 0
for a in range(1, ceil(K**(1/3))+1):
    for b in range(a, ceil((K/a)**0.5)+1):
        res += K % (a * b) == 0 and b <= K/a/b

print(res)
