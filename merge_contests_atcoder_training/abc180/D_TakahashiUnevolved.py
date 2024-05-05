#        D - Takahashi Unevolved
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc180/tasks/abc180_d
# ----------------------------------------

# 強さをなるべく増やさないように操作したい
# ある点を超えると、A倍にする操作ができなくなる -> それ以降はB加算を続ける

x, y, a, b = map(int, input().split())

cnt = 0
while x * a <= x + b < y:
    x *= a
    cnt += 1

cnt += (y - x - 1) // b
print(cnt)
