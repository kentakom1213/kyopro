from itertools import permutations

N, M = map(int, input().split())
S = [input() for _ in range(N)]

def diff(s, t):
    d = 0
    for x, y in zip(s, t):
        d += x != y
    return d

for p in permutations(S):
    is_ok = True
    for i in range(N - 1):
        is_ok &= diff(p[i], p[i + 1]) == 1
    
    if is_ok:
        print("Yes")
        exit()

print("No")
