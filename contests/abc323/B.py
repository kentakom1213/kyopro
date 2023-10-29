N = int(input())
SS = [input() for _ in range(N)]

wins = [s.count("o") for s in SS]

res = sorted(range(N), key=lambda i: (-wins[i], i))

print(*map(lambda i: i + 1, res))
