
N = int(input())
S = input()

visited = set()

cur = (0, 0)
visited.add(cur)

for c in S:
    cx, cy = cur
    if c == "R":
        cur = (cx + 1, cy)
    elif c == "L":
        cur = (cx - 1, cy)
    elif c == "U":
        cur = (cx, cy + 1)
    else:
        cur = (cx, cy - 1)
    
    if cur in visited:
        print("Yes")
        exit()

    visited.add(cur)

print("No")
