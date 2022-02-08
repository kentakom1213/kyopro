#          B - おとぎの国の高橋君
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc009/tasks/arc009_2

# PyPy3だとMLE
# ----------------------------------------

B = input().split()
order = {b:str(i) for i, b in enumerate(B)}
N = int(input())

A = [0] * N
for i in range(N):
    a = input()
    replaced = "".join(order[c] for c in a)
    A[i] = (int(replaced), a)

A.sort()
for r, a in A:
    print(a)
