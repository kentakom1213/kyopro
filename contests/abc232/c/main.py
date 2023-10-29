
# N <= 8だから全探索っぽい
# 8! = 40320

from itertools import permutations

N, M = map(int, input().split())

G = [[] for _ in range(N)]
for _ in range(M):
    a, b = map(int, input().split())
    a -= 1
    b -= 1
    G[a].append(b)
    G[b].append(a)

# ソート
for v in G:
    v.sort()


points = [tuple(map(lambda x: x-1, map(int, input().split()))) for _ in range(M)]

# G1をG2に寄せる

items = list(range(N))
perm = list(range(N-1, -1, -1))

isOK = False

for perm in permutations(items):
    repl = {x:y for x, y in zip(items, perm)}

    newG = [[] for _ in range(N)]

    for c, d in points:
        x = repl[c]
        y = repl[d]

        newG[x].append(y)
        newG[y].append(x)
    
    for v in newG:
        v.sort()
    
    isOK |= G == newG

print("Yes" if isOK else "No")