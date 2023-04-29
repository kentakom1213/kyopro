H, W = map(int, input().split())
A = [input() for _ in range(H)]
B = [input() for _ in range(H)]

def shift(s, t):
    """
    Aを縦にs回、横にt回シフトしたものがBと等しいか
    """
    tmp = True
    for i in range(H):
        for j in range(W):
            r = (i + s) % H
            c = (j + t) % W
            tmp &= A[r][c] == B[i][j]
    return tmp

def main():
    is_ok = False
    for s in range(H):
        for t in range(W):
            is_ok |= shift(s, t)
    
    print("Yes" if is_ok else "No")

main()