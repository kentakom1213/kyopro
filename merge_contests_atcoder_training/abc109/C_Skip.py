#               C - Skip
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc109/tasks/abc109_c
# ----------------------------------------

def gcd(a, b): return gcd(b, a%b) if b else a

N, X = map(int, input().split())
A = [int(v) - X for v in input().split()]

ans = 0
for a in A:
    ans = gcd(ans, a)

print(abs(ans))
