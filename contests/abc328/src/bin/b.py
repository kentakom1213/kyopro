N = int(input())
D = list(map(int, input().split()))

ans = 0

for i in range(N):
    if len(set(str(i + 1))) > 1:
        continue
    tmp = str(i + 1)[0]
    for j in range(1, 20):
        if len(set(tmp)) == 1 and int(tmp * j) <= D[i]:
            # print(i + 1, tmp * j)
            ans += 1

print(ans)
