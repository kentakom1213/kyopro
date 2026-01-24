
# AC

A, B, C = map(int, input().split())

for n in range(1, 1000):
    nC = C * n
    if A <= nC <= B:
        print(nC)
        break
else:
    print(-1)