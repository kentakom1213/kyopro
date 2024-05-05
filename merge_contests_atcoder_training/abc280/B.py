N = int(input())
S = list(map(int, input().split()))

A = [0] * N
A[0] = S[0]

for i in range(N-1):
    A[i+1] = S[i+1] - S[i]

print(*A)
