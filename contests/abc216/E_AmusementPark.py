#        E - Amusement Park
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc216/tasks/abc216_e
# ----------------------------------------

# B = (1,2,...A1,1,2,...A2,...A3,...AN)
# から大きい順にK個選べば良い

# => 数列Bに含まれるm以上の値はK個以下か?

N, K = map(int, input().split())
A = list(map(int, input().split()))

