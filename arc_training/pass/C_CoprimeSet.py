#            C - Coprime Set
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc118/tasks/arc118_c
# ----------------------------------------

N = int(input())

print(2, 3, end=" ")

for i in range(2, N):
    print(2*i, end=" ")
