
H, W = map(int, input().split())
A = [tuple(map(int, input().split())) for _ in range(H)]

TA = list(zip(*A))

for i in range(W):
    for j in range(H):
        print(TA[i][j], end=" ")
    print()
