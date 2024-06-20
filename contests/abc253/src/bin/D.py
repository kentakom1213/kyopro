def gcd(a, b): return gcd(b, a%b) if b else a
def lcm(a, b): return a // gcd(b, a%b) * b

def sum_(n):
    return n * (n+1) // 2

n, a, b = map(int, input().split())

lcm_ab = lcm(a, b)

sum_a = a * sum_(n // a)
sum_b = b * sum_(n // b)
sum_ab = lcm_ab * sum_(n // lcm_ab)

if a == b:
    ans = sum_(n) - sum_a
    print(ans)
else:
    ans = sum_(n) - (sum_a + sum_b - sum_ab)
    print(ans)
