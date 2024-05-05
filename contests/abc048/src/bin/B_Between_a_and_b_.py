#        B - Between a and b ...
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc048/tasks/abc048_b
# ----------------------------------------

a, b, x = map(int, input().split())
l = -(-a//x)
r = b//x
print(r-l+1)
