#              C - A x B + C
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc179/tasks/abc179_c

# AC
# ----------------------------------------

# A * B + C = N を満たす (A, B, C) の組を求めよ

N = int(input())

eq, not_eq = 0, 0
for a in range(1, N+1):
    for b in range(a, N+1):
        if a * b >= N: break

        if a == b: eq += 1
        else: not_eq += 1

print(eq + 2 * not_eq)