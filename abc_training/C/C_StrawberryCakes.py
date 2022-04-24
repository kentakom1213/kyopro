#        C - Strawberry Cakes
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/ddcc2020-qual/tasks/ddcc2020_qual_c
# ----------------------------------------

"""
## [解説](https://maspypy.com/atcoder-%e5%8f%82%e5%8a%a0%e6%84%9f%e6%83%b3-2019-11-23ddcc2020%e4%ba%88%e9%81%b8#toc3)
- いちごに番号をつけ、縦横に拡大していく
"""

H, W, K = map(int, input().split())
S = [input() for _ in range(H)]

# いちごに番号をつける
field = [[0]*W for _ in range(H)]
cnt = 1
for i in range(H):
    for j in range(W):
        if S[i][j] == '#':
            field[i][j] = cnt
            cnt += 1

# いちごの番号を横に拡大
for i in range(H):
    # 0だったら左の数を採用
    for j in range(1, W):
        if field[i][j] == 0:
            field[i][j] = field[i][j-1]
    
    # 0だったら右の数を採用
    for j in range(W-2, -1, -1):
        if field[i][j] == 0:
            field[i][j] = field[i][j+1]

# いちごの番号を縦に拡大
for j in range(W):
    # 0だったら左の数を採用
    for i in range(1, H):
        if field[i][j] == 0:
            field[i][j] = field[i-1][j]
    
    # 0だったら右の数を採用
    for i in range(H-2, -1, -1):
        if field[i][j] == 0:
            field[i][j] = field[i+1][j]

for row in field:
    print(*row, sep=" ")
