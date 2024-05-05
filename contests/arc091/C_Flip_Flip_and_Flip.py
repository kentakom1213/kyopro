# C - Flip,Flip, and Flip......
# -----------------------------
# 問題
# https://atcoder.jp/contests/arc091/tasks/arc091_a?lang=ja
# -----------------------------

# まずはシミュレーション

# [1, 1, 1, 1, 1]
# [1, 1, 1, 1, 1]
# [1, 1, 1, 1, 1]
# [1, 1, 1, 1, 1]
# [1, 1, 1, 1, 1]
#  ↓
# [1, 1, 1, 1, 1]
# [1, 0, 0, 0, 1]
# [1, 0, 0, 0, 1]
# [1, 0, 0, 0, 1]
# [1, 1, 1, 1, 1]

# [1, 1, 1, 1, 1]
#  ↓
# [1, 0, 0, 0, 1]

# import sys
# def exprint(x): print(*x, sep="\n", file=sys.stderr)
# from itertools import product
# AROUND = list(product((-1, 0, 1), repeat=2))
# 
# N, M = map(int, input().split())
# array = [[1] * M for _ in range(N)]
# 
# exprint(array)
# print()
# 
# for r in range(N):
#     for c in range(M):
#         # 反転
#         for dr, dc in AROUND:
#             if 0 <= r+dr < N and 0 <= c+dc < M:
#                 array[r+dr][c+dc] ^= 1
# 
#         exprint(array)
#         print()

# 実装
N,M=map(int,input().split())
print(N*M-2 if bool(N-1)^bool(M-1)else (N-2)*(M-2))


