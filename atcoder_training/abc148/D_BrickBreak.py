#           D - Brick Break
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc148/tasks/abc148_d
# ----------------------------------------

# 1, 2, 3, ... と続く数列を探す

N = int(input())
A = list(map(int, input().split()))

now = 1
for a in A:
    if now == a:
        now += 1

ans = N - now + 1
print(ans if ans != N else -1)
