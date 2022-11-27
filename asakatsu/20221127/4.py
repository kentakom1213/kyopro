N, M = map(int, input().split())
A = list(map(int, input().split()))
BC = [list(map(int, input().split())) for _ in range(M)]

A.sort()
BC.sort(key=lambda x: x[1])

ans = 0
for a in A:
    if BC and BC[-1][1] > a:
            ans += BC[-1][1]
            BC[-1][0] -= 1
            if BC[-1][0] == 0:
                BC.pop()
    else:
        ans += a
print(ans)