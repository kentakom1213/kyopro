#           C - Dubious Document          
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc058/tasks/arc071_a
# ----------------------------------------

from collections import Counter

N = int(input())
S = [Counter(input()) for _ in range(N)]

# 共通部分を求める
common = S[0]
for s in S:
    for k, v in common.items():
        common[k] = min(s[k], v)

# 共通部分をlistに直す
res = sorted([k * v for k, v in common.items()])

print("".join(res))
