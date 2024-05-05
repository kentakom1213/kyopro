# E - Sorting Queries
# WA

from bisect import bisect_left
import array
from collections import deque

Q = int(input())
queries = [input() for _ in range(Q)]

A_sorted = deque()

for q in queries:
    if q == "2":
        print(A_sorted.popleft())
    elif q[0] == "1":
        num = int(q.split()[1])
        A_sorted.insert(bisect_left(A_sorted, num), num)
