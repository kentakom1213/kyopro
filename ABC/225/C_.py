
# AC

if __name__ == "__main__":
    N, M = map(int, input().split())
    B = [list(map(int, input().split())) for _ in range(N)]

    start = B[0][0]

    isOK = True
    for i in range(N):
        for j in range(M):

            if j < M-1:
                isOK &= B[i][j] % 7 != 0
            
            isOK &= B[i][j] == start + 7 * i + j
            # print(isOK, start + 7 * i + j)

    print("Yes" if isOK else "No")