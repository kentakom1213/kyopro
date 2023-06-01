A, B, K = map(int, input().split())

res = []
for i in range(1, 1_000_000):
    if A % i == 0 and B % i == 0:
        res.append(i)

res.sort()
print(res[-K])
