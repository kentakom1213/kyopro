H, W = map(int, input().split())
C = [input() for _ in range(H)]

SIZ = min(H, W)

def get_X_size(r, c):
    """
    (r, c)を中心とするばつ印の大きさを返す
    """

    ok = [True] * (SIZ + 1)

    for i in range(SIZ + 1):
        top = r - i
        btm = r + i
        left = c - i
        right = c + i

        if top < 0 or btm >= H or left < 0 or right >= W:
            ok[i] = False
            break

        ok[i] &= C[top][left] == "#" 
        ok[i] &= C[top][right] == "#"
        ok[i] &= C[btm][left] == "#"
        ok[i] &= C[btm][right] == "#"

    res = 0
    for i in range(SIZ + 1):
        if ok[i]:
            res = i
        else:
            break
    return res

def main():
    ans = [0] * (SIZ + 1)

    for r in range(H):
        for c in range(W):
            siz = get_X_size(r, c)
            ans[siz] += 1
        
    print(*ans[1:])

main()
