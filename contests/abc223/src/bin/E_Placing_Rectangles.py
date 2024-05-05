#          E - Placing Rectangles         
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc223/tasks/abc223_e
# ----------------------------------------

def div_ceil(a, b):
    if b <= 0:
        return 1e20
    return (a + b - 1) // b


def split_1(X, Y, A, B, C):
    """
    縦に3分割できるか判定する
    """
    y1 = div_ceil(A, X)
    y2 = div_ceil(B, X)
    y3 = div_ceil(C, X)
    return y1 + y2 + y3 <= Y


def split_2(X, Y, A, B, C):
    """
    縦に分割したあと，残りを横に分割する
    """
    y1 = div_ceil(A, X)
    y_rem = Y - y1  # Aを切り取った残り
    x1 = div_ceil(B, y_rem)
    x2 = div_ceil(C, y_rem)
    return x1 + x2 <= X


def main():
    X, Y, A, B, C = map(int, input().split())

    is_ok = False

    for _ in range(2):
        is_ok |= split_1(X, Y, A, B, C)
        # A, B, Cのどれを上に持ってくるか
        is_ok |= split_2(X, Y, A, B, C)
        is_ok |= split_2(X, Y, B, A, C)
        is_ok |= split_2(X, Y, C, A, B)
        # 縦横を交換してもう一度試す
        X, Y = Y, X

    print("Yes" if is_ok else "No")


main()
