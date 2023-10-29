

N, M = list(map(int, input().split()))
C = input().split()
D = input().split()
P = list(map(int, input().split()))

value = {}

for d, p in zip(D, P[1:]):
    value[d] = p

ans = 0

for c in C:
    if c in value:
        ans += value[c]
    else:
        ans += P[0]

print(ans)
