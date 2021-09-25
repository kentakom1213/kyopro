#            B - Many Oranges
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc195/tasks/abc195_b
# ----------------------------------------

A, B, W = map(int, input().split())
W *= 1000

div_B, mod_B = W // B, W % B
if A <= mod_B <= B:
    min_num = div_B + 1
elif mod_B == 0:
    min_num = div_B
else:
    min_num = None

div_A, mod_A = W // A, W % A
if mod_A <= B - A:
    max_num = div_A
else:
    max_num = div_A + 1

print(min_num, max_num)