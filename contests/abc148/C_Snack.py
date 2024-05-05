#               C - Snack
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc148/tasks/abc148_c
# ----------------------------------------

def gcd(a, b): return gcd(b, a%b) if b else a
def lcm(a, b): return a // gcd(b, a%b) * b
a, b = map(int, input().split())
print(lcm(a, b))
