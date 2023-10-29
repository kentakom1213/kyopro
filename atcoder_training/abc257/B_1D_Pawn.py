#               B - 1D Pawn               
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc257/tasks/abc257_b
# ----------------------------------------

N, K, Q = map(int, input().split())
A = [int(n) for n in input().split()]
L = [int(n)-1 for n in input().split()]

for q in L:
    if A[q] == N:
        continue
    elif q == K-1 or A[q]+1 < A[q+1]:
        A[q] += 1

print(*A)
