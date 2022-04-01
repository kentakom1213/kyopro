#              C - Lining Up              
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc050/tasks/arc066_a

# これで緑diffかあ
# ----------------------------------------

from collections import Counter
MOD = 1000000007

N = int(input())
A = list(map(int, input().split()))

cnt = Counter(A)
ans = 1

if N % 2 == 1 and cnt[0] >= 2:
    ans = 0

for d, n in cnt.items():
    if n > 2 or N%2 == d%2:
        ans = 0
        break
    ans = ans * n % MOD

print(ans)
