from fractions import Fraction

N = int(input())

A = []
B = []
for _ in range(N):
    a, b = map(int, input().split())
    A.append(a)
    B.append(b)

P = [Fraction(a, a + b) for a, b in zip(A, B)]

ans = sorted(range(N), key=lambda i: -P[i])

print(*(i + 1 for i in ans), sep=" ")
