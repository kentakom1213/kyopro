N, L, W = map(int, input().split())
A = list(map(int, input().split())) + [L]

now = cnt = 0
for x in A:
    if now < x:
        span = x - now
        cnt += (span + W - 1) // W  # 今後ceilはこれを使おう
    now = x + W

print(cnt)
