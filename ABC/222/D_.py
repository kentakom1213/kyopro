
MOD = 998244353

N = int(input())
A = list(map(int, input().split()))
B = list(map(int, input().split()))

res = [1] * N
for i in range(N):
    a, b = A[i], B[i]
    for j in range(a, b+1):
        if i == 0:
            res = b-a+1
        else:
            res[i] += 