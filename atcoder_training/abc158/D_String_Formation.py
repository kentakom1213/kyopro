#           D - String Formation          
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc158/tasks/abc158_d
# ----------------------------------------

from audioop import reverse
from collections import deque

dq = deque(input())
head = True
Q = int(input())

for _ in range(Q):
    x = input()
    if x == "1":
        head ^= 1
    else:
        _, f, c = x.split()
        if head ^ (f == "1"):
            dq.append(c)
        else:
            dq.appendleft(c)

if head:
    print("".join(dq))
else:
    print("".join(reversed(dq)))
