#               B - ニコニコ文字列               
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/dwango2015-prelims/tasks/dwango2015_prelims_2
# ----------------------------------------

from collections import deque

def comb2(n):
    return n * (n+1) // 2

S = input()

q = deque()
smiley = []

for c in S:
    if (not q or q[-1] == "5") and c == "2":
        q.append(c)
    elif q and q[-1] == "2" and c == "5":
        q.append(c)
    else:
        smiley.append(len(q))
        q.clear()
        if c == "2":
            q.append(c)
else:
    smiley.append(len(q))

ans = 0
for i in smiley:
    ans += comb2(i // 2)

print(ans)
