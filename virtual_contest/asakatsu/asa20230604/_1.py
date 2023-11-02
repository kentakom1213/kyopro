N = int(input())
A = list(map(int, input().split()))
B = list(map(int, input().split()))

max_A = [0] * (N + 1)
max_AB = [0] * (N + 1)

for i in range(N):
    max_A[i + 1] = max(max_A[i], A[i])
    max_AB[i + 1] = max(max_AB[i], max_A[i + 1] * B[i])

print(*max_AB[1:])
