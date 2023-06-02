# https://atcoder.jp/contests/abc253/tasks/abc253_d

def gcd(a, b): return gcd(b, a%b) if b else a
def lcm(a, b): return a // gcd(b, a%b) * b
def sigma(n): return n * (n+1) // 2

N, A, B = map(int, input().split())

ans = sigma(N)
ans -= sigma(N//A) * A + sigma(N//B) * B

ab = lcm(A, B)
ans += sigma(N//ab) * ab

print(ans)
