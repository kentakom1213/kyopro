#           A - A to Z String 2           
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc257/tasks/abc257_a
# ----------------------------------------

from string import ascii_uppercase
n, x = map(int, input().split())
print(ascii_uppercase[(x-1)//n])