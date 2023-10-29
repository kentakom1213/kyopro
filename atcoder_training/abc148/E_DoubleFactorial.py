#         E - Double Factorial
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc148/tasks/abc148_e
# ----------------------------------------

# N!! の末尾の0の個数
# Nが偶数のとき、素因数に含まれる5の数を調べればいい

N = int(input())

if N & 1:
    print(0)
    exit()

cnt = 0
i = 5
while i <= N:
    cnt += N // i // 2
    i *= 5

print(cnt)
