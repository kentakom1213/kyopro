
# AC

def game():
    p, q = map(int, input().split())
    a, b, c = map(int, input().split())

    S = set()

    acom, pany = 1, 1
    while acom and pany:
        p = (a * p + b) % c
        if p in S:
            acom = 0
        
        q = (a * q + b) % c
        if q in S:
            pany = 0
        
        S.add(p)
        S.add(q)
    
    if acom:
        return "Acom"
    elif pany:
        return "Pany"
    else:
        return "Draw"


if __name__ == "__main__":
    t = int(input())

    res = [0] * t
    for i in range(t):
        res[i] = game()

    print(*res, sep="\n")