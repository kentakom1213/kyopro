# https://atcoder.jp/contests/abc052/tasks/arc067_a

from collections import defaultdict
MOD = 1_000_000_007

N = int(input())

# N!を素因数分解
def factoring_factorial(N):
    result = defaultdict(int)
    for n in range(2, N+1):
        for i in range(2, int(n ** 0.5 + 1)):
            if n % i != 0: continue
            while n % i == 0:
                n //= i
                result[i] += 1
        if n != 1:
            result[n] += 1
    return result

# 素因数分解
facts = factoring_factorial(N)

ans = 1
for k, v in facts.items():
    ans = (ans * (v + 1)) % MOD

print(ans)
