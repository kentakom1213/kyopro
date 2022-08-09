#          B - Tournament Result          
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc261/tasks/abc261_b
# ----------------------------------------

N = int(input())
A = [input() for _ in range(N)]

isOK = True
for i in range(N):
    for j in range(i+1, N):
        Aij = A[i][j]
        Aji = A[j][i]
        if Aij == Aji == "D":
            continue
        if (Aij, Aji) == ("W", "L"):
            continue
        if (Aij, Aji) == ("L", "W"):
            continue
        else:
            isOK = False
print("correct" if isOK else "incorrect")
