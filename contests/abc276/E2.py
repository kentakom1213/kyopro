
def dfs()


if __name__ == "__main__":
    
    H, W = map(int, input().split())
    field = [input() for _ in range(H)]
    G = [[] for _ in range(H*W)]

    s = -1
    for i in range(H-1):
        for j in range(W-1):
            v = H*i + j
            if H[i][j] == "S":
                s = v
            if H[i][j] == "#":
                continue
            if H[i+1][j] != "#":
                nv = H*(i+1) + j
                G[v].append(nv)
                G[nv].append(v)
            if H[i+1][j] != "#":
                nv = H*i + j+1
                G[v].append(nv)
                G[nv].append(v)
            if H[i+1][j] != "#":
                nv = H*(i+1) + j+1
                G[v].append(nv)
                G[nv].append(v)



    if len(ans) >= 4:
        print("Yes")
    else:
        print("No")
