#              E - Round Trip             
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc276/tasks/abc276_e
# ----------------------------------------

import sys
sys.setrecursionlimit(1000000)
from itertools import combinations

def yes():
    print("Yes")
    exit()

def no():
    print("No")
    exit()


if __name__ == "__main__":

    H, W = map(int, input().split())
    field = [input() for _ in range(H)]

    sr = sc = 0
    for i in range(H):
        for j in range(W):
            if field[i][j] == "S":
                sr, sc = i, j

    # `S`に隣接するマスを生成
    adj_cells = []

    for i, j in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
        nr, nc = sr+i, sc+j
        if i==j==0 \
        or not 0 <= nr < H \
        or not 0 <= nc < W \
        or field[nr][nc] == "#":
            continue

        adj_cells.append((nr, nc))

    # 隣接するセルの組合せについて全探索
    for a, b in combinations(adj_cells, 2):
        
    
    no()
    
