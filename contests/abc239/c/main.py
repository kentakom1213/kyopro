
x1, y1, x2, y2 = map(int, input().split())

dx, dy = abs(x1 - x2), abs(y1 - y2)

OK = [
    (1, 1),
    (2, 0), (0, 2),
    (1, 3), (3, 1),
    (3, 3),
    (2, 4), (4, 2),
    (0, 4), (4, 0)
]

isOK = (dx, dy) in OK

print("Yes" if isOK else "No")
