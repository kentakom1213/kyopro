
N = int(input())
H = list(map(int, input().split()))

now = -1e10
for h in H:
    if now >= h:
        break
    now = h

print(now)
