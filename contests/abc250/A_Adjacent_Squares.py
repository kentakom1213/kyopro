#           A - Adjacent Squares          
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc250/tasks/abc250_a
# ----------------------------------------

h, w = map(int, input().split())
r, c = map(int, input().split())

DR = [0, 1, 0, -1]
DC = [1, 0, -1, 0]

ans = 0
for i in range(4):
    nr = r + DR[i]
    nc = c + DC[i]
    ans += 1 <= nr <= h and 1 <= nc <= w

print(ans)
