from math import ceil

N, L, W = map(int, input().split())
A = list(map(int, input().split())) + [L]

now = cnt = 0
for x in A:
    if now < x:
        span = x - now
        cnt += span // W
        if span % W:
            cnt += 1
    now = x + W

print(cnt)
