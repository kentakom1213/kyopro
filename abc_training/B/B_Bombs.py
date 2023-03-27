#                B - Bombs                
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc295/tasks/abc295_b
# ----------------------------------------

R, C = map(int, input().split())
B = [list(input()) for _ in range(R)]

for i in range(R):
    for j in range(C):
        if B[i][j] in ".#": continue
        d = int(B[i][j])
        for x in range(R):
            for y in range(C):
                if abs(i - x) + abs(j - y) <= d:
                    if B[x][y] == "#":
                        B[x][y] = "."
                    B[i][j] = "."

for r in B:
    print(*r, sep="")