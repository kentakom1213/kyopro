# https://atcoder.jp/contests/abc065/tasks/arc076_a

MOD = 1000000007

def factorial(n):
    res = 1
    for i in range(1, n + 1):
        res = res * i % MOD
    return res

N, M = map(int, input().split())

res = 0

match abs(N - M):
    case 0:
        res = factorial(N) * factorial(M) * 2 % MOD
    case 1:
        res = factorial(N) * factorial(M) % MOD
    case _:
        pass

print(res)
