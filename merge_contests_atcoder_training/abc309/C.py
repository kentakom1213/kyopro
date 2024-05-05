N, K = map(int, input().split())

A = [0] * N
B = [0] * N

summ = 0
for i in range(N):
    A[i], B[i] = map(int, input().split())
    summ += B[i]

A.append(0)
B.append(0)

# Aをソート
AB = sorted(zip(A, B))

for a, b in AB:
    summ -= b
    if summ <= K:
        print(a + 1)
        break
