# https://atcoder.jp/contests/abc202/tasks/abc202_c

from collections import Counter

N = int(input())
A = Counter(map(int, input().split()))
B = list(map(int, input().split()))
C = list(map(int, input().split()))

ans = 0
for j in range(N):
    ans += A[B[C[j]-1]]
print(ans)
