#            D - Opposite
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc197/tasks/abc197_d

# 初めて競プロで回転行列を使った
# ----------------------------------------

import numpy as np
from numpy import sin, cos, pi
np.set_printoptions(suppress=True)

N = int(input())
top = np.array([int(v) for v in input().split()])
bottom = np.array([int(v) for v in input().split()])

# 中心
O = (top + bottom) / 2

# 回転行列
t = - 2*pi / N
A = np.array([
    [cos(t), -sin(t)],
    [sin(t), cos(t)]
])

P1 = (top - O) @ A + O

print(*P1)
