#            C - Invisible Hand           
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc312/tasks/abc312_c
# ----------------------------------------

from bisect import bisect_left, bisect_right

N, M = map(int, input().split())
A = sorted(map(int, input().split()))
B = sorted(map(int, input().split()))

ng = 0
ok = 1e20

while ok - ng > 1:
    mid = (ng + ok) // 2

    # Aのmid未満の要素
    sell = bisect_right(A, mid)

    # Bのmid以上の要素
    buy = M - bisect_left(B, mid)

    if sell >= buy:
        ok = mid
    else:
        ng = mid

print(int(ok))
