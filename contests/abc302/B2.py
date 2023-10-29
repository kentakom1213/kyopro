snuke = "snuke"

H, W = map(int, input().split())
S = [input() for _ in range(H)]

DIR = [
    (0, 1),
    (1, 0),
    (-1, 0),
    (0, -1),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
]

for i in range(H):
    for j in range(W):
        for di, dj in DIR:
            tmp = ""
            for k in range(5):
                r = i + di * k
                c = j + dj * k
                if 0 <= r < H and 0 <= c < W:
                    tmp += S[r][c]
                else:
                    break
            if tmp == snuke:
                for k in range(5):
                    print(
                        i + di * k + 1,
                        j + dj * k + 1
                    )
                exit()
