#              B - Visibility             
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc197/tasks/abc197_b
# ----------------------------------------

H, W, X, Y = map(int, input().split())
X, Y = X-1, Y-1
F = [input() for _ in range(H)]

# 行について
tmpY = 0
is_contain_y = False
for j in range(W):
    if F[X][j] == ".":
        tmpY += 1
    else:
        if is_contain_y:
            break
        tmpY = 0
    if j == Y:
        is_contain_y = True

# 列について
tmpX = 0
is_contain_x = False
for i in range(H):
    if F[i][Y] == ".":
        tmpX += 1
    else:
        if is_contain_x:
            break
        tmpX = 0
    if i == X:
        is_contain_x = True

# 重複を除く
print(tmpY + tmpX - 1)
