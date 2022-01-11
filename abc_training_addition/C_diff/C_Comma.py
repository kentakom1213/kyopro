#                C - Comma
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc195/tasks/abc195_c

# AC
# ----------------------------------------

N = int(input())

# 方針
# d桁の数に書くコンマの数は (d-1)//3 個

d = len(str(N))

counter = 0
for i in range(1, d):
    counter += ((i-1) // 3) * (10**i - 10**(i-1))

counter += ((d-1) // 3) * (N - 10**(d-1) + 1)

print(counter)