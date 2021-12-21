
def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

def get_ans():
    # input
    n, k = map(int, input().split())
    P = list(map(int, input().split()))

    G = init_array(n, n, 0)
    for i in range(n):
        a, b = map(int, input().split())
        a -= 1
        b -= 1
        G[a][b] = 1
        G[b][a] = 1

    # solve
    


if __name__ == "__main__":
    t = int(input())
    res = [0] * t

    for i in range(t):
        res[i] = get_ans()
    
    print(*res, sep="\n")