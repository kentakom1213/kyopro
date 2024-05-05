#         C - Convex Quadrilateral        
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc266/tasks/abc266_c
# ----------------------------------------

# 方針
# --------------------
# 全ての頂点が、それ以外の3点からなる3角形の外部に存在する
# --------------------

def sub(a, b):
    return (a[0]-b[0], a[1]-b[1])

def sign(a, b):
    return a[0]*b[1] - a[1]*b[0] > 0

def isContain(p, abc):
    a, b, c = abc

    # ベクトルを求める
    ab = sub(a, b)
    bp = sub(b, p)

    bc = sub(b, c)
    cp = sub(b, p)

    ca = sub(c, a)
    ap = sub(a, p)

    # 外積
    c1 = sign(ab, bp)
    c2 = sign(bc, cp)
    c3 = sign(ca, ap)

    # 判定
    return c1 == c2 == c3


if __name__ == "__main__":
    P = [tuple( map(int, input().split() ) ) for _ in range(4)]

    isOK = True
    for i in range(4):
        p = P[i]
        abc = P[:i] + P[i+1:]
        isOK &= not isContain(p, abc)

    print("Yes" if isOK else "No")
