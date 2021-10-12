# C - Reorder Cards
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc213/tasks/abc213_c

# 普通の座圧でした。昨日のABC218-Dやっといてよかった

# AC
# ----------------------------------------

from bisect import bisect_left

H, W, N = map(int, input().split())
AB = [tuple(map(int, input().split())) for _ in range(N)]

vals_a = sorted(set( a for a, b in AB ))
vals_b = sorted(set( b for a, b in AB ))

for a, b in AB:
    print(
        bisect_left(vals_a, a) + 1,
        bisect_left(vals_b, b) + 1
        )
