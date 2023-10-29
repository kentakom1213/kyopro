
H, W = map(int, input().split())
S = [input() for _ in range(H)]

os = []
for i in range(H):
    for j in range(W):
        if S[i][j] == "o":
            os.append((i, j))

a, b = os[0]
c, d = os[1]
print(abs(a-c) + abs(b-d))
