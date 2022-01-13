# B - Two Arrays
# ---------------
# 問題
# https://atcoder.jp/contests/apc001/tasks/apc001_b

# 解説
# https://img.atcoder.jp/apc001/editorial.pdf
# ---------------

# A, Bの差をとって考える
# diffのうち2項に+2, -1を同時に加える
# 操作回数は -sum(diff) 回になる
# この回数で実現可能かを考えよう

from math import ceil

N = int(input())
A = list(map(int, input().split()))
B = list(map(int, input().split()))

cnt = 0
for a, b in zip(A, B):
    if a > b:
        cnt += a - b
    if a < b:
        cnt += (a - b) // 2

print("Yes" if cnt < 0  else "No")

