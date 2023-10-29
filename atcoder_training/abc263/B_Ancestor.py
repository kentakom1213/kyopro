#               B - Ancestor              
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc263/tasks/abc263_b
# ----------------------------------------

N = int(input())
P = list(map(int, input().split()))

ans = 0
parent = N - 2
while parent >= 0:
    parent = P[parent] - 2
    ans += 1
print(ans)
