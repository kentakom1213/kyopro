N, M = map(int, input().split())

S = [input() for _ in range(N)]
T = [input() for _ in range(M)]

ans = 0
for s in S:
    isOK = False
    for t in T:
        if s[-3:] == t:
            isOK |= True
    ans += isOK

print(ans)
