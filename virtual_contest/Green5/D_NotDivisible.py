# https://atcoder.jp/contests/abc170/tasks/abc170_d

from collections import Counter

N = int(input())
A = list(map(int, input().split()))

MAX_SIZE = 1001001
exi = [0] * MAX_SIZE

for a_i in A:
    exi[a_i] += 1

cnt = Counter(A)
for a, c in cnt.items():
    if c >= 2:
        exi[a] = 0

for a, c in cnt.items():
    for j in range(2, MAX_SIZE):
        if a * j >= MAX_SIZE:
            break

        exi[a * j] *= 0

print(sum(exi))