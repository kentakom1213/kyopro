# https://atcoder.jp/contests/agc046/tasks/agc046_a

def gcd(a, b): return gcd(b, a%b) if b else a
def lcm(a, b): return a // gcd(b, a%b) * b

x = int(input())
lcm360 = lcm(x, 360)

print(lcm360 // x)
