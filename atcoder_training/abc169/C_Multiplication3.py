#          C - Multiplication 3
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc169/tasks/abc169_c

# AC
# ----------------------------------------

# if __name__ == "__main__":
#     # ↓WA 桁落ち誤差をどう処理するか
#     A, B = input().split()
#     A = int(A)
#     # print(int(A * B))  # WA: 桁落ち誤差をどう処理するか

#     B_digits = int(B[0]), int(B[2]), int(B[3])
#     x, y, z = map(str, (A * b for b in B_digits))

#     upper = int(x)
#     upper += int(y[:-1]) if len(y) > 1 else 0
#     upper += int(z[:-2]) if len(z) > 2 else 0

#     lower = int(y[-1]) * 10 + int(z[-2:])

#     res = upper + (1 if lower >= 100 else 0)
#     print(res)


# 解説
# そううまくはいかないみたい
if __name__ == "__main__":
    In = input().split()
    mul100 = int(In[0]) * int(float(In[1]) * 100)
    print(mul100 // 100)