# 084 - There are two types of characters（★3）
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/typical90/tasks/typical90_cf
# ----------------------------------------

# oxoxo
# ↓ランレングス圧縮
# 1 1 1 1 1
#   1 1 1 1  | 4
#     1 1 1  | 3
#       1 1  | 2
#         1  | 1
#            + 10

# xxoooxx
# 2 3 2
#   6 6
#     2


from itertools import groupby
N = int(input())
S = input()

rle = [len(list(group)) for key, group in groupby(S)]

S = [0] * len(rle)
S[0] = rle[0]
for i in range(1, len(rle)):
    S[i] = S[i-1] + rle[i]

res = 0
for i in range(len(rle)-1):
    res += S[i]

print(res)
