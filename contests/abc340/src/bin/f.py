X, Y = map(int, input().split())


def ext_gcd(a, b):
    """
    `a*x + b*y = gcd(a, b) = d`
    となる `(x, y, d)` を返す
    """
    if b == 0:
        return 1, 0, a

    q, r = divmod(a, b)
    x, y, d = ext_gcd(b, r)
    s, t = y, x - q * y
    return s, t, d


isX = X < 0
isY = Y < 0

# 第一象限にもってくる
X = abs(X)
Y = abs(Y)

a, b, d = ext_gcd(-Y, X)

# 戻す
if isX:
    a = -a
if isY:
    b = -b

if abs(d) == 1:
    print(2*a, 2*b)
elif abs(d) == 2:
    print(a, b)
else:
    print(-1)
