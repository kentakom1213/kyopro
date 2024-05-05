#              C - 単調増加
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc038/tasks/abc038_c
# ----------------------------------------

from collections import deque
def com2(n):
    return (n+1) * n // 2

N = int(input())
A = list(map(int, input().split()))

inc = []

sub = deque()
top = 0
for i, a in enumerate(A):
    if not sub or sub[-1] < a:
        sub.append(a)
    else:
        inc.append(len(sub))
        sub = deque()
        sub.append(a)
inc.append(len(sub))

ans = sum(map(com2, inc))
print(ans)
