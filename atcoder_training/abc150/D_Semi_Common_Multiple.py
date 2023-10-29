#         D - Semi Common Multiple
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc150/tasks/abc150_d
# ----------------------------------------

def gcd(a, b): return gcd(b, a % b) if b else a
def lcm(a, b): return a // gcd(b, a % b) * b


N, M = map(int, input().split())
A = list(map(int, input().split()))

# a/2の最小公倍数を求める
x = 1
for a in A:
    x = lcm(x, a // 2)
    # if x > M:
    #     print(0)
    #     exit()

ans = int((M / x + 1) // 2)
print(ans)
