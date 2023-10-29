#         C - String Invasion
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc113/tasks/arc113_c

# AC
# ----------------------------------------

from collections import defaultdict

S = input()
N = len(S)

ans = 0

chars = defaultdict(int)
for i in range(1, N):
    a, b = S[-1-i], S[-i]
    
    if a == b:
        ans += i - 1 - chars[a]
        chars = defaultdict(int)
        chars[a] = i
    else:
        chars[b] += 1

print(ans)
