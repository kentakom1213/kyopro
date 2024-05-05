N, M = map(int, input().split())
S = [input() for _ in range(N)]

test = [[0] * 9 for _ in range(9)]

# 全探索
def isOK(r, c):
    """行、列"""
    is_ok = True

    # 黒
    for i in range(3):
        for j in range(3):
            is_ok &= S[r + i][c + j] == "#"
            is_ok &= S[r + i + 6][c + j + 6] == "#"
            # test[r + i][c + j] = "#"
            # test[r + i + 6][c + j + 6] = "#"
    
    # 白
    for i in range(4):
        is_ok &= S[r + 3][c + i] == "."
        is_ok &= S[r + i][c + 3] == "."
        is_ok &= S[r + 5][c + 5 + i] == "."
        is_ok &= S[r + 5 + i][c + 5] == "."
        # test[r + 3][c + i] = "."
        # test[r + i][c + 3] = "."
        # test[r + 5][c + 5 + i] = "."
        # test[r + 5 + i][c + 5] = "."
    
    # print(r, c, is_ok)
    return is_ok


# isOK(0, 0)
# print(*test, sep="\n")

for r in range(N - 8):
    for c in range(M - 8):
        if isOK(r, c):
            print(r + 1, c + 1)
