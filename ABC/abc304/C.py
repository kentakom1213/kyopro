from collections import deque

N, D = map(int, input().split())
XY = [list(map(int, input().split())) for _ in range(N)]

def can_flu(x, y):
    a1, b1 = XY[x]
    a2, b2 = XY[y]
    return (a1 - a2) ** 2 + (b1 - b2) ** 2 <= D ** 2

# N^2回計算
is_flu = [False] * N
is_flu[0] = True

q = deque([0])
while q:
    u = q.popleft()
    for v in range(N):
        if is_flu[v]:
            continue
        if can_flu(u, v):
            q.append(v)
            is_flu[v] = True
    
for b in is_flu:
    print("Yes" if b else "No")
