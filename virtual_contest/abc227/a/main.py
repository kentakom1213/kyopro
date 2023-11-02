
N, K, A = map(int, input().split())

now = A-1
for _ in range(K):
    if now < N:
        now += 1
    else:
        now = 1

print(now)