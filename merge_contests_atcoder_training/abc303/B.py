N, M = map(int, input().split())
A = [list(map(int, input().split())) for _ in range(M)]

hunaka = [[0] * N for _ in range(N)]

for row in range(M):
    for i in range(N - 1):
        l = A[row][i] - 1
        r = A[row][i + 1] - 1
        hunaka[l][r] += 1
        hunaka[r][l] += 1
    
cnt = 0
for i in range(N):
    for j in range(i + 1, N):
        cnt += hunaka[i][j] == 0

print(cnt)
