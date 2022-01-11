#              C - Made Up
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc202/tasks/abc202_c

# AC
# ----------------------------------------

N = int(input())
A = list( map(lambda x: x-1, map(int, input().split()) ) )
B = list( map(lambda x: x-1, map(int, input().split()) ) )
C = list( map(lambda x: x-1, map(int, input().split()) ) )

BC_i = [0] * N
for i in range(N):
    BC_i[ B[C[i]] ] += 1

counter = 0
for i in range(N):
    counter += BC_i[ A[i] ]

print(counter)