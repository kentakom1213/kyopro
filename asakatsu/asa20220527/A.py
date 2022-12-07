# https://atcoder.jp/contests/abc192/tasks/abc192_b

from curses.ascii import isalpha

s = input()

isok = True
for i, c in enumerate(s, start=1):
    if i & 1:
        isok &= c.isalpha() and c.islower()
    else:
        isok &= c.isalpha() and c.isupper()

print("Yes" if isok else "No")
