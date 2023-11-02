# https://atcoder.jp/contests/agc028/tasks/agc028_a

def gcd(a, b): return gcd(b, a%b) if b else a
def lcm(a, b): return a // gcd(b, a%b) * b

N, M = map(int, input().split())
S, T = input(), input()


