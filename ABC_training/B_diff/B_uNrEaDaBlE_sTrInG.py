#          B - uNrEaDaBlE sTrInG
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc192/tasks/abc192_b

# AC
# ----------------------------------------

from string import ascii_lowercase, ascii_uppercase

S = input()

for i, c in enumerate(S):
    if i % 2 == 0:
        if c in ascii_uppercase:
            print("No")
            break
    else:
        if c in ascii_lowercase:
            print("No")
            break
else:
    print("Yes")

