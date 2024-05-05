#               B - A^B^C
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc113/tasks/arc113_b

# AC
# ----------------------------------------

# A^B^Cの1の位

A, B, C = map(int, input().split())

# A^kの1桁目をdictに保存
powA = A % 10
mod10 = set()
while powA not in mod10:
    mod10.add(powA)
    powA = (powA * A) % 10

loop = len(mod10)
n = pow(B, C, loop)

res = A % 10
for _ in range(n + loop - 1):
    res = (res * A) % 10

print(res)
