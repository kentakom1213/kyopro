#         B - Template Matching
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc054/tasks/abc054_b
# ----------------------------------------

import numpy as np

N, M = map(int, input().split())
A_in = [list(map(lambda c:c=="#", input())) for _ in range(N)]
B_in = [list(map(lambda c:c=="#", input())) for _ in range(M)]

A = np.array(A_in)
B = np.array(B_in)

def is_same(a, b):
    return not (a ^ b).any()

for i in range(N-M+1):
    for j in range(N-M+1):
        if is_same(A[i:i+M, j:j+M], B):
            print("Yes")
            exit()

print("No")
