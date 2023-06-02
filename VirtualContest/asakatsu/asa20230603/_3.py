# https://atcoder.jp/contests/arc010/tasks/arc010_2

from itertools import product

N = int(input())
A = [int(v) for v in input().split()]

ans = 0
for diff in product((-1, 0, 1), repeat=N):
    is_ok = False
    for a, d in zip(A, diff):
        is_ok |= (a + d) % 2 == 0
    ans += is_ok

print(ans)
