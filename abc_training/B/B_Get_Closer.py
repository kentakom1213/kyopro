#              B - Get Closer             
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc246/tasks/abc246_b
# ----------------------------------------

a, b = map(int, input().split())

l = pow(a**2+b**2, 1/2)
print(a/l, b/l)
