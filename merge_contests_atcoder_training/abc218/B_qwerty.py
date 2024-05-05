#                B - qwerty
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc218/tasks/abc218_b

# AC
# ----------------------------------------

from string import ascii_lowercase

P = list(map(int, input().split()))

res = ""
for p in P:
    res += ascii_lowercase[p-1]

print(res)