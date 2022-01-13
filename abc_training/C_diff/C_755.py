# C - 755
# --------
# 問題
# https://atcoder.jp/contests/abc114/tasks/abc114_c

# 解説
# https://blog.hamayanhamayan.com/entry/2018/12/02/231238
# --------

# 10^9以下の753数の数は3^9 = 19683 しかない
# 全探索しても十分間に合う

from itertools import product

N = int(input())

# 753数を生成
cnt = 0
digits = tuple("357")
for i in range(1, 10):
    for num in product(digits, repeat=i):
        if all(d in num for d in digits) and int("".join(num)) <= N :
            cnt += 1

print(cnt)

