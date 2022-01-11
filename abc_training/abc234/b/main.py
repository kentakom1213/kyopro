
N = int(input())
points = [tuple(map(int, input().split())) for _ in range(N)]

max_len = 0
for i in range(N):
    for j in range(i+1, N):
        x0, y0 = points[i]
        x1, y1 = points[j]
        length = (x0 - x1)**2 + (y0 - y1)**2
        max_len = max(max_len, length)

print(max_len ** 0.5)