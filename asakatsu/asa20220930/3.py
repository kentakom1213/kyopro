# https://atcoder.jp/contests/keyence2019/tasks/keyence2019_c

from collections import deque

N = int(input())
A = list(map(int, input().split()))
B = list(map(int, input().split()))

diff = [a-b for a, b in zip(A, B)]

if sum(diff) < 0:
    print(-1)
    exit()

diff.sort()

ans = 0
x = 0
for i in range(N):
    if diff[i] >= 0:
        break
    ans += 1
    x += diff[i]

for i in range(N-1, -1, -1):
    if x >= 0:
        break
    x += diff[i]
    ans += 1

print(ans)
