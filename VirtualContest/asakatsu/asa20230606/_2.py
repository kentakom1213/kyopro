x1, y1, x2, y2 = map(int, input().split())

def dist2(a, b):
    x1, y1 = a
    x2, y2 = b
    return (x1 - x2) ** 2 + (y1 - y2) ** 2

q = []

# 1の候補
for i in range(-4, 4):
    for j in range(-4, 4):
        q.append((x1 + i, y1 + j))

# 2の候補
for i in range(-4, 4):
    for j in range(-4, 4):
        q.append((x2 + i, y2 + j))

# qで条件を満たすものがあるか
a = (x1, y1)
b = (x2, y2)
for c in q:
    if dist2(a, c) == 5 and dist2(b, c) == 5:
        print("Yes")
        exit()

print("No")
