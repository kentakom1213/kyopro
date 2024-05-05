from random import randint

N = 14
MAX = 20

def plot(all, choose):
    field = [['_'] * (MAX + 1) for _ in range(MAX + 1)]
    for x, y in all:
        field[x][y] = 'x'
    for x, y in choose:
        field[x][y] = 'o'
    for row in field:
        print(''.join(row))
    print()

def is_included(U, V, X) -> bool:
    """
    点Xが点U,Vによる凸包に支配されているかを判定
    """

    a1, b1 = U
    a2, b2 = V
    x, y = X

    # 点に支配されている
    if (x <= a1 and y <= b1) or (x <= a2 and y <= b2):
        return True

    # 線分に支配されている
    m = a1 * b2 - a2 * b1

    if m == 0:
        return False

    l = (a1 * y - b1 * x) / m
    k = - (a2 * y - b2 * x) / m

    return (
        0 <= l <= 1
        and 0 <= k <= 1
        and l + k <= 1
    )


P = set()
while len(P) < N:
    P.add((randint(0, MAX), randint(0, MAX)))

P = list(P)

plot(P, P)

convexes = set()

for i in range(1 << N):
    choice = []
    for j in range(N):
        if i >> j & 1:
            choice.append(P[j])

    # choiceに束縛される点集合を求める
    convex = []
    for X in P:
        is_in = False
        for u in range(len(choice)):
            for v in range(u, len(choice)):
                is_in |= is_included(choice[u], choice[v], X)
        if is_in:
            convex.append(X)

    convexes.add(tuple(convex))

convexes_sorted = sorted(convexes, key=len)

for c in convexes_sorted:
    plot(P, c)

print(len(convexes))
