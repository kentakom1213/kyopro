
from collections import deque

N, M = map(int, input().split())
S = input().split()
T = deque(input().split())

for s in S:
    if T and T[0] == s:
        print("Yes")
        T.popleft()
    else:
        print("No")
