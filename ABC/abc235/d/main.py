import sys
sys.setrecursionlimit(10000)
import sys
def err(*args, **kwargs): print(*args, **kwargs, file=sys.stderr)
from functools import lru_cache

a, N = map(int, input().split())

cnt = 1e10

@lru_cache
def reverse_(n, depth=0):
    global cnt
    if n < 1 or depth > 200:
        return
    if n == 1:
        cnt = min(cnt, depth)
        return
    if n > 10 and str(n)[-1] != 0:
        new = int(str(n)[-1] + str(n)[:-1])
        reverse_(new, depth+1)
    if n % a == 0:
        reverse_(n//a, depth+1)


reverse_(N)
print(cnt if cnt < 1e10 else -1)
