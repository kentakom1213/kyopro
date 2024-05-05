#              C - Shapes
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc218/tasks/abc218_c

# AC
# ----------------------------------------

def exprint(x): print(*x, sep="\n")

def extractShape(field):
    res = []
    for i in range(N):
        for j in range(N):
            if field[i][j] == "#":
                res.append((i, j))
    return res

def turnRight(field):
    # 上下反転
    field = field[::-1]
    # 転地
    res = [""] * N
    for i in range(N):
        for j in range(N):
            res[i] += field[j][i]
    
    return res

def isParallel(s, t):
    s_len , t_len= len(s), len(t)
    if s_len != t_len:
        return False

    diff = s[0][0] - t[0][0], s[0][1] - t[0][1]
    for i in range(1, s_len):
        diff_i = s[i][0] - t[i][0], s[i][1] - t[i][1]
        if diff != diff_i:
            return False
    
    return True

if __name__ == "__main__":
    N = int(input())
    field_S = [input() for _ in range(N)]
    field_T = [input() for _ in range(N)]

    for n in range(4):
        shape_S = extractShape(field_S)
        shape_T = extractShape(field_T)

        # print()
        # print(shape_S, shape_T)
        # exprint(field_S)
        # exprint(field_T)

        if isParallel(shape_S, shape_T):
            print("Yes")
            break
        else:
            field_S = turnRight(field_S)
    else:
        print("No")