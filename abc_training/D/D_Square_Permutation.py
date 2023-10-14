#          D - Square Permutation         
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc324/tasks/abc324_d
# ----------------------------------------

import math

N = input()
S = input()

# 平方数を列挙
sq = {}

MAX = 3500000
for i in range(1, MAX):
    n = "".join(sorted(str(i * i)))
    if n in sq:
        sq[n] += 1
    else:
        sq[n] = 1

# ソート
S = "".join(sorted(S))
ans = 0
for _ in range(len(S) + 1):
    tmp = sq.get(S, 0)
    ans += tmp

    if S[0] == "0":
        S = S[1:]
    else:
        break

print(ans)

