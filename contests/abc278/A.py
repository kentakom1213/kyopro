N, K = map(int, input().split())
A = list(map(int, input().split()))

print(*(A[K:] + [0]*K)[:N])
