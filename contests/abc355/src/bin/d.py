from heapq import heapify, heappop

n = int(input())
q = []
for i in range(n):
    l, r = map(int, input().split())
    q.append((l, -1))
    q.append((r, 1))

heapify(q)

ans = 0
now = 0
while q:
    _, y = heappop(q)

    if y == -1:
        ans += now
        now += 1
    else:
        now -= 1

print(ans)
