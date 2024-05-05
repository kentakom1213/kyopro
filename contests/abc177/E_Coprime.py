#               E - Coprime
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc177/tasks/abc177_e

# AC
# ----------------------------------------

# N <= 10^6
# Ai <= 10^6

# pairwise: Aiの範囲が狭いことを利用し、ふるいで処理
# setwise: gcdをreduceで回す

from functools import reduce
def gcd(a, b): return gcd(b, a%b) if b else a

N = int(input())
A = list(map(int, input().split()))
A.sort()
A_MAX = A[-1] + 1

## pairwiseについて
is_pairwise = True

sieve = [0] * A_MAX
for n in A:
    sieve[n] = 1

for i in range(2, A_MAX):
    cnt = 0
    for j in range(1, (A_MAX-1)//i+1):
        cnt += sieve[i*j]
    is_pairwise &= cnt < 2

## setwiseについて
is_setwise = reduce(gcd, A) == 1

if is_pairwise and is_setwise:
    print("pairwise coprime")
elif is_setwise:
    print("setwise coprime")
else:
    print("not coprime")
