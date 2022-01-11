#           C - ABC conjecture
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc227/tasks/abc227_c
# ----------------------------------------

N = int(input())

res = 0
a = 1
while a**3 <= N:
    b = a
    while a*b*b <= N:
        res += N // (a * b) - b + 1

        b += 1
    a += 1

print(res)
