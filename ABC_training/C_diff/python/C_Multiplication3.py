#          C - Multiplication 3
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc169/tasks/abc169_c

# 
# ----------------------------------------

if __name__ == "__main__":
    # ↓WA 桁落ち誤差をどう処理するか
    A, B = input().split()
    A = int(A)
    # print(int(A * B))  # WA: 桁落ち誤差をどう処理するか

    B_digits = int(B[0]), int(B[2]), int(B[3])
    x, y, z = map(str, (A * b for b in B_digits))

    res = int(x) 
    res += int(y[:-1]) if len(y) >= 1 else 0 
    res += int(z[:-2]) if len(z) >= 2 else 0
    print(res)