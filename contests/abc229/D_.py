

from collections import deque

S = input()
K = int(input())

res = now = 0

q = deque()

for c in S:
    if c == "X":
        q.append(c)
        now += 1

    if c == ".":
        if K == 0:
            while q:
                now -= 1
                if q.popleft() == ".":
                    K += 1
                    break

        if K:
            q.append(c)
            K -= 1
            now += 1
    
    # print(q, K)  # test
    res = max(res, now)

print(res)