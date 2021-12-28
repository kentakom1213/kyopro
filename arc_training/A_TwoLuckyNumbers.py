#         A - Two Lucky Numbers
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc131/tasks/arc131_a
# ----------------------------------------

# A, B = 2, 5
# -> x = 25, 2x = 50

A = input()
B = int(input())

if B % 2 == 0:
    print(A + str(B//2))
else:
    print(A + str(B//2) + "5")