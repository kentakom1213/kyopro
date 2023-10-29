#               B - Batters               
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc256/tasks/abc256_b
# ----------------------------------------

N = int(input())
A = list(map(int, input().split()))

P = 0
B = [0] * 4

for a in A:
    B[0] = 1
    P += sum(B[4-a:])
    B = [0]*a + B[:4-a]
print(P)
