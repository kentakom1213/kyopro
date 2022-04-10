#           B - Unique Nicknames          
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc247/tasks/abc247_b

# 確かにむずい
# ----------------------------------------

from collections import defaultdict

cnt = defaultdict(int)
names = []

N = int(input())
for _ in range(N):
    m, n = input().split()
    if m != n:
        cnt[m] += 1
        cnt[n] += 1
    else:
        cnt[m] += 1

    names.append((m, n))

# 貪欲に処理
for m, n in names:
    if cnt[m] > 1 and cnt[n] > 1:
        print("No")
        exit()

print("Yes")
