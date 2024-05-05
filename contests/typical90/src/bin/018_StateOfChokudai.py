#      018 - Statue of Chokudai（★3）
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/typical90/tasks/typical90_r
# ----------------------------------------

### めんどくせえええええ

import math

def phi(e):
    theta = 2 * math.pi * (e % T) / T
    p = L * math.sin(theta) / 2
    q = L * (1 - math.cos(theta)) / 2

    phi = math.asin(q / math.sqrt(X**2 + (Y - p)**2 + q**2))
    phi_360 = 180 * phi / math.pi
    return phi_360



if __name__ == "__main__":
    T = int(input())
    L, X, Y = map(int, input().split())

    Y *= -1  # 設計ミスにより追加

    Q = int(input())
    E = [int(input()) for _ in range(Q)]

    for e in E:
        print(phi(e))
