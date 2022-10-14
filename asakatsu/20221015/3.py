# https://atcoder.jp/contests/zone2021/tasks/zone2021_d

from collections import deque

S = input()

deq = deque()
is_back = True

for c in S:
    if c == "R":
        is_back ^= True
    else:
        if is_back:
            if deq and deq[-1] == c:
                deq.pop()
            else:
                deq.append(c)
        else:
            if deq and deq[0] == c:
                deq.popleft()
            else:
                deq.appendleft(c)

if is_back:
    print("".join(deq))
else:
    print("".join(reversed(deq)))
