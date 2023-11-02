# https://atcoder.jp/contests/agc012/tasks/agc012_a

N = int(input())
A = list(map(int, input().split()))

A.sort()
ans = sum(A[N:2*N])
print(ans)
