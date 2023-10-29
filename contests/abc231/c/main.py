
from bisect import bisect_left

N, Q = map(int, input().split())
A = list(map(int, input().split()))

A.sort()

for i in range(Q):
    q = int(input())

    ok = bisect_left(A, q)
    print(N - ok)