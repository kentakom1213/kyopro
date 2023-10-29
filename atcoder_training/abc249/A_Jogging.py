#               A - Jogging               
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc249/tasks/abc249_a
# ----------------------------------------

a, b, c, d, e, f, x = map(int, input().split())

t = x // (a+c) * a * b + b * min(a, x%(a+c))
a = x // (d+f) * d * e + e * min(d, x%(d+f))

if t > a:
    print("Takahashi")
elif t == a:
    print("Draw")
else:
    print("Aoki")

