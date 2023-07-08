
a, b = map(int, input().split())

is_ok = (a, b) in [
    (1, 2),
    (2, 3),
    (4, 5),
    (5, 6),
    (7, 8),
    (8, 9),
]

print("Yes" if is_ok else "No")

# X = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]

