N, M = map(int, input().split())

X = [[] for _ in range(M)]
for i in range(M):
    k, *X[i] = map(int, input().split())

isOK = True
for i in range(1, N+1):
    for j in range(i+1, N+1):
        tmp = False
        for row in X:
            tmp |= (i in row) and (j in row)
        isOK &= tmp

print("Yes" if isOK else "No")