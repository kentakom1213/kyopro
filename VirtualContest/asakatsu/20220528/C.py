
N = int(input())
A = list(map(int, input().split()))

ans = [0] * N

for i in range(N):
    if i+1 < N:
        if A[i] > A[i+1]:
            ans[i] ^= 1
            ans[i+1] ^= 1

print(*ans)