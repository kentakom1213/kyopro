
# AC

import sys
input = sys.stdin.readline

M = 998244353

T = int(input())
for _ in range(T):

    a, b, c, n = map(int, input().split())

    if a == 1 or a == M + 1:
        print((c + b*n) % M)
    else:
        x = (c - b * pow(1-a, M-2, M) % M) * pow(a, n, M) % M + b * pow(1-a, M-2, M) % M
        print(x % M)
