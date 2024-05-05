#           C - Compass Walking
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc198/tasks/abc198_c

# 細かい調整に手間取った

# AC
# ----------------------------------------

import math

R, X, Y = map(int, input().split())

distance = (X ** 2 + Y ** 2) ** 0.5

print(math.ceil(distance / R) if R <= distance else 2)
