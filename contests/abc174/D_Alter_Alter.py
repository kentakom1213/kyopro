# D - Alter Alter
# ----------------
# 問題
# https://atcoder.jp/contests/abc174/tasks/abc174_d

# 嘘解法？？

# AC
# ----------------

N = int(input())
C = list(input())

cnt = 0

# 左右から尺取り
l, r = 0, N-1
while l < r:
    if (C[l], C[r]) == ("R", "W"):
        l += 1
        r -= 1
    elif (C[l], C[r]) == ("W", "R"):
        C[l], C[r] = C[r], C[l]
        cnt += 1
        l += 1
        r -= 1
    elif C[l] == "W":
        r -= 1
    elif C[r] == "R":
        l += 1
    print(C)

print(cnt)

