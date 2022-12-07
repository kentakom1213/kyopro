import math
A, B = map(int, input().split())

def f(x):
    return B * x + A / math.sqrt(x+1)

# 二分探索
l, r = -1, 1e18
while (r - l) > 2:
    c1 = l + (r - l) // 3
    c2 = r - (r - l) // 3
    if f(c1) < f(c2):
        r = c2
    else:
        l = c1

res = (l + r) // 2
print(f(res))
