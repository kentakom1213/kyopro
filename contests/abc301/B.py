N = int(input())
A = list(map(int, input().split()))

ans = []

for i in range(N - 1):
    ans.append(A[i])
    if A[i] + 1 < A[i + 1]:
        for j in range(A[i] + 1, A[i + 1]):
            ans.append(j)
    if A[i + 1] + 1 < A[i]:
        for j in range(A[i] - 1, A[i + 1], -1):
            ans.append(j)

ans.append(A[-1])

print(*ans)
