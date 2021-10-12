#            C - Min Difference
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc212/tasks/abc212_c

# AC
# ----------------------------------------

N, M = map(int, input().split())
A_sorted = sorted(list(map(int, input().split())))
B_sorted = sorted(list(map(int, input().split())))

a, b = A_sorted.pop(), B_sorted.pop()
min_diff = abs(a - b)

while A_sorted or B_sorted:
    min_diff = min(min_diff, abs(a - b))
    if not A_sorted:
        b = B_sorted.pop()
    elif not B_sorted:
        a = A_sorted.pop()
    elif a < b:
        b = B_sorted.pop()
    else:
        a = A_sorted.pop()

print(min_diff)