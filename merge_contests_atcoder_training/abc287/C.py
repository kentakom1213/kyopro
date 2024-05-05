
N, M = map(int, input().split())
deg = [0] * N

for i in range(M):
    a, b = map(int, input().split())
    deg[a-1] += 1
    deg[b-1] += 1

cnt_1 = 0
cnt_2 = 0
for d in deg:
    cnt_1 += d == 1
    cnt_2 += d == 2

isOK = cnt_1 == 2 \
    and cnt_2 == N-2 \
    and N-1 == M

print("Yes" if isOK else "No")
