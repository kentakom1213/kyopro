#            B - ロボットアーム
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/mujin-pc-2016/tasks/mujin_pc_2016_b

# AC
# ----------------------------------------

import math

if __name__ == "__main__":
    a, b, c = map(int, input().split())

    # 最大半径
    maxi = a + b + c
    mini = 0 if a + b - c >= 0 and a + c - b >= 0 and b + c - a >= 0 \
        else min(map(abs, [
        a + b - c,
        a - b + c,
        -a + b + c,
    ]))

    print(math.pi * (maxi**2 - mini**2))