#           C - Multiple Gift
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc083/tasks/arc088_a

# シンプルに実装できた
# AC
# ----------------------------------------

# input
X, Y = map(int, input().split())

# solve
counter = 0
while X <= Y:
    X *= 2
    counter += 1

print(counter)

