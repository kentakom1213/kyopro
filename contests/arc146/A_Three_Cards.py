#             A - Three Cards             
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc146/tasks/arc146_a
# ----------------------------------------

from itertools import permutations

N = int(input())
A = input().split()

# 選択
A.sort(key=lambda x: (len(x), x), reverse=True)
choose = A[:3]

# 順列の全探索
ans = 0
for perm in permutations(choose):
    ans = max(
        ans,
        int( "".join(perm) )
    )

print(ans)
