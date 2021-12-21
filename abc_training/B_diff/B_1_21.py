#                B - 1 21
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc086/tasks/abc086_b

# AC
# ----------------------------------------

a, b = input().split()

x = int(a + b) ** 0.5

if x - int(x) < 1e-5:
    print("Yes")
else:
    print("No")