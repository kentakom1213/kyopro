#          Koto Municipal Subway          
# ----------------------------------------
# 問題
# https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=2699&lang=jp
# ----------------------------------------

import math

def solve():
    """一つのテストケースを解く"""
    d, e = map(int, input().split())

    if (d, e) == (0, 0):
        exit()

    # x, yのペアを全探索すれば良い
    mini = 1e20

    for i in range(-200, 201):
        x, y = i, d - i
        score = abs(math.sqrt(x ** 2 + y ** 2) - e)

        if x >= 0 and y >= 0 and score < mini:
            mini = score
    
    # 最適値の差を計算
    print(mini)


def main():
    while True:
        solve()

main()
