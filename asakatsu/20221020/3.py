# https://atcoder.jp/contests/abc253/tasks/abc253_d

def gcd(a, b): return gcd(b, a%b) if b else a
def lcm(a, b): return a // gcd(b, a%b) * b

def sum_n(n):
    return n * (n+1) // 2

N, A, B = map(int, input().split())

# 1~N
ans = sum_n(N)

# Aの倍数
ans -= sum_n(N//A) * A

# Bの倍数
ans -= sum_n(N//B) * B

# AかつBの倍数
ans += sum_n(N//lcm(A, B)) * lcm(A, B)

print(ans)
