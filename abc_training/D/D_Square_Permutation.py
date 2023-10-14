#          D - Square Permutation
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc324/tasks/abc324_d
# ----------------------------------------

N = input()
S = "".join(sorted(input()))

MAX = 3500000
ans = 0

for i in range(MAX):
    n = "".join(sorted(str(i * i)))
    if len(n) > len(S):
        break
    else:
        n = "0" * (len(S) - len(n)) + n

    if n == S:
        ans += 1

print(ans)
