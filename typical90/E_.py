


if __name__ == "__main__":
    N, M = map(int, input().split())
    G = [[0]*N for _ in range(N)]
    for i in range(M):
        a, b = map(int, input().split())
        a -= 1
        b -= 1
        G[a][b] = 1
        G[b][a] = 1

    print(*G, sep="\n")
