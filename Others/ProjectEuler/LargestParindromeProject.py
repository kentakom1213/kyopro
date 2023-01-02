#       Largest palindrome product
# ----------------------------------------
# 問題
# https://projecteuler.net/problem=4
# ----------------------------------------

#     100*100 <= n <= 999*999
# <=> 10000 <= n <= 998001 < 10^6

max_n = (0, 0, 0)
for i in range(100, 1000):
    for j in range(100, 1000):
        n = i*j
        if str(n) == str(n)[::-1]:
            if max_n[0] < n:
                max_n = (n, i, j)

print(max_n)
