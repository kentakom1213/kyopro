#              A - Full House             
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc263/tasks/abc263_a
# ----------------------------------------

from collections import Counter
cnt = Counter(map(int, input().split()))
print("Yes" if (len(cnt)==2 and 2 in cnt.values()) else "No")
