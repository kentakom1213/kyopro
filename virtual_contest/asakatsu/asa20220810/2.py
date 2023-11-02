# https://atcoder.jp/contests/arc120/tasks/arc120_a

N = int(input())
A = list(map(int, input().split()))

# max of A / sum of A
max_A = [0] * N
sum_A = [0] * N
max_A[0] = sum_A[0] = A[0]
for i in range(1, N):
    max_A[i] = max(max_A[i-1], A[i])
    sum_A[i] = sum_A[i-1] + A[i]

# solve
accum = 0
for i in range(N):
    accum += sum_A[i]
    res = max_A[i] * (i+1) + accum
    print(res)