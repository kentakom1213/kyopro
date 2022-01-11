#                 C - POW
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc205/tasks/abc205_c

# 普通に簡単

# AC
# ----------------------------------------

A, B, C = map(int, input().split())

if C % 2 == 0:
    if abs(A) == abs(B): print("=")
    elif abs(A) > abs(B): print(">")
    else: print("<")
else:
    if A == B: print("=")
    elif A > B: print(">")
    else: print("<")