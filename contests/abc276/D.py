from functools import reduce
from collections import defaultdict

def factoring(n):
    result = defaultdict(int)
    for i in range(2, int(n ** 0.5 + 1)):
        if n % i != 0: continue
        while n % i == 0:
            n //= i
            result[i] += 1
    if n != 1:
        result[n] += 1
    return result

def gcd(a, b): return gcd(b, a%b) if b else a
def lcm(a, b): return a // gcd(b, a%b) * b

N = int(input())
A = list(map(int, input().split()))
    
# 最小公倍数
gcd_a = reduce(gcd, A)

# 全て最大公約数で割る
A = [a//gcd_a for a in A]

ans = 0
for facs in map(factoring, A):
    # Aが2,3以外の素因数を持つ場合を排除
    if not set(facs.keys()) <= {2, 3}:
        print(-1)
        exit()

    for v in facs.values():
        ans += v

print(ans)
