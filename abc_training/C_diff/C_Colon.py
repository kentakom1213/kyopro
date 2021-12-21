#             C - : (Colon)
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc168/tasks/abc168_c

# こういうの時間かけずに解きたい
# ----------------------------------------

import math

if __name__ == "__main__":
    a, b, h, m = map(int, input().split())

    # 角度を求めて余弦定理でOK
    phi = 2 * math.pi * abs( m - (5*h + m/12) ) / 60

    res = math.sqrt( a**2 + b**2 - 2*a*b * math.cos(phi) )

    print(res)
    