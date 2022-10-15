
"""
## 方針
- 座標圧縮

## 参考
- https://drken1215.hatenablog.com/entry/2021/08/09/235400
"""

from bisect import bisect_left

H, W, R, C = map(int, input().split())

# 壁
N = int(input())
r_s = [0]*N + [-1, 1e9]
c_s = [0]*N + [-1, 1e9]
for i in range(N):
    r_s[i], c_s[i] = map(int, input().split())

# 圧縮
r_comp = sorted(set(r_s))
c_comp = sorted(set(c_s))

# フィールドの構築
field = [[0]*(N+2) for _ in range(N+2)]
for i in range(N+2):
    field[i][0] = 1
    field[0][i] = 1
    field[i][N+1] = 1
    field[N+1][i] = 1

for i in range(N):
    r, c = r_s[i], c_s[i]
    r_idx = bisect_left(r_comp, r)
    c_idx = bisect_left(c_comp, c)
    field[r_idx][c_idx] = 1

# デバッグ
# print("R", r_comp)
# print("C", c_comp)
# print(*field, sep="\n")

# 移動コマンド
MOVE = {
    "L": (0, -1),
    "R": (0, 1),
    "U": (1, 0),
    "D": (-1, 0),
}

# 移動のシミュレート
Q = int(input())
for i in range(Q):
    d, l = input().split()
    l = int(l)

    C_idx = bisect_left(c_comp, C)
    R_idx = bisect_left(r_comp, R)

    if d == "L":
        while l and field[C_idx][R_idx] == 0:
            C_idx = bisect_left(c_comp, C)
            move = min(l, C - c_comp[C_idx - 1])
            C -= move
            l -= move
            # print("L", move)
    elif d == "R":
        while l and field[C_idx][R_idx] == 0:
            C_idx = bisect_left(c_comp, C)
            move = min(l, c_comp[C_idx + 1] - C)
            C += move
            l -= move
            # print("R", move)
    elif d == "U":
        while l and field[C_idx][R_idx] == 0:
            R_idx = bisect_left(r_comp, R)
            move = min(l, R - r_comp[R_idx - 1])
            R -= move
            l -= move
            # print("U", move)
    elif d == "D":
        while l and field[C_idx][R_idx] == 0:
            R_idx = bisect_left(r_comp, R)
            move = min(l, r_comp[R_idx + 1] - R)
            R += move
            l -= move
            # print("D", move)
    print(R, C)
