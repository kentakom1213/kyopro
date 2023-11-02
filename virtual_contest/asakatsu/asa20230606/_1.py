field = [input() for _ in range(8)]

for r, x in enumerate("87654321"):
    for c, y in enumerate("abcdefgh"):
        if field[r][c] == "*":
            print(y + x)
