#              D - Polyomino              
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc322/tasks/abc322_d
# ----------------------------------------

P1 = [input() for _ in range(4)]
P2 = [input() for _ in range(4)]
P3 = [input() for _ in range(4)]

# 配列に変換
arr1 = [tuple(map(lambda c: int(c == '#'), row)) for row in P1]
arr2 = [tuple(map(lambda c: int(c == '#'), row)) for row in P2]
arr3 = [tuple(map(lambda c: int(c == '#'), row)) for row in P3]

def rotate90(arr):
    new = [row[:] for row in arr]
    return list(zip(*new[::-1]))

def slide(arr):
    for dr in range(-4, 4):
        for dc in range(-4, 4):
            res = [[0] * 4 for _ in range(4)]
            flag = True
            for r in range(4):
                for c in range(4):
                    nr = r + dr
                    nc = c + dc
                    if arr[r][c] and (nr >= 4 or nr < 0 or nc >= 4 or nc < 0):
                        flag = False
                        break
                    res[(r + dr) % 4][(c + dc) % 4] = arr[r][c]
            if flag:
                yield res
            
def is_filled(a, b, c):
    ok = True
    s = 0
    for i in range(4):
        for j in range(4):
            ok &= a[i][j] or b[i][j] or c[i][j]
            s += a[i][j] + b[i][j] + c[i][j]
    return ok and s == 16

# 全探索
isok = False
for _ in range(3):
    for a in slide(arr1):
        for _ in range(3):
            for b in slide(arr2):
                for _ in range(3):
                    for c in slide(arr3):
                        isok |= is_filled(a, b, c)
                    arr3 = rotate90(arr3)
            arr2 = rotate90(arr2)
    arr1 = rotate90(arr1)

print("Yes" if isok else "No")
