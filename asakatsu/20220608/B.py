# https://atcoder.jp/contests/abc231/tasks/abc231_c

from bisect import bisect_left

N, Q = map(int, input().split())
A = list(map(int, input().split()))
A.sort()

for i in range(Q):
    x = int(input())
    idx = bisect_left(A, x)
    print(N - idx)
