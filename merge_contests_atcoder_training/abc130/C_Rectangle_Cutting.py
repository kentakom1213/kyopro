#          C - Rectangle Cutting          
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc130/tasks/abc130_c
# ----------------------------------------

W, H, x, y = map(int, input().split())
print(H*W/2, 1 if (x, y) == (W/2, H/2) else 0)
