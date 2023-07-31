#               B - TaK Code
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc312/tasks/abc312_b
# ----------------------------------------

import numpy as np

N, M = map(int, input().split())

# True/Falseで受けとり
S = [list(map(lambda x: x == '#', input())) for _ in range(N)]

# numpy配列に変換
A = np.array(S)

# 無限大
x = int(1e9)

# パターン
ok = np.array(
    [
        [1, 1, 1, 0, x, x, x, x, x],
        [1, 1, 1, 0, x, x, x, x, x],
        [1, 1, 1, 0, x, x, x, x, x],
        [0, 0, 0, 0, x, x, x, x, x],
        [x, x, x, x, x, x, x, x, x],
        [x, x, x, x, x, 0, 0, 0, 0],
        [x, x, x, x, x, 0, 1, 1, 1],
        [x, x, x, x, x, 0, 1, 1, 1],
        [x, x, x, x, x, 0, 1, 1, 1],
    ]
)

# 全探索
for r in range(N - 8):
    for c in range(M - 8):
        target = A[r:r+9, c:c+9]
        xor = target ^ ok
        xor[xor > 1] = 0
        
        if xor.sum() == 0:
            print(r + 1, c + 1)
