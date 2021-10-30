
# AC

if __name__ == "__main__":
    N = int(input())

    G = [[] for _ in range(N)]
    for _ in range(N-1):
        a, b = map(int, input().split())
        a -= 1
        b -= 1

        G[a].append(b)
        G[b].append(a)

    vs = list(map(len, G))
    
    if len(list(filter(lambda x: x == 1, vs))) == N-1 and max(vs) == N-1:
        print("Yes")
    else:
        print("No")

