
H, W = map(int, input().split())
field = [input() for _ in range(H)]

X = [0] * W
for i in range(H):
    for j in range(W):
        X[j] += field[i][j] == "#"

print(*X)
