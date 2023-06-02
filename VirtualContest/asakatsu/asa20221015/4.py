# https://atcoder.jp/contests/dwango2015-prelims/tasks/dwango2015_prelims_2

from collections import deque

def comb2(n):
    return n * (n+1) // 2

S = input()

# "25" -> "X"
s = deque(S)
deq = deque()

while s:
    top = s.popleft()
    if top == "2":
        deq.append(top)
    elif deq and deq[-1] == "2" and top == "5":
        deq[-1] = "X"
    else:
        deq.append(top)

# ランレングス圧縮
comp = []
now = deq[0]
cnt = 0
for c in deq:
    if now == c:
        cnt += 1
    else:
        comp.append((now, cnt))
        now = c
        cnt = 1
comp.append((now, cnt))

# comb2
ans = 0
for c, v in comp:
    if c == "X":
        ans += comb2(v)

print(ans)