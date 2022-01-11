#               B - 時計版
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc030/tasks/abc030_b
# ----------------------------------------

n, m = map(int, input().split())

hour = (n % 12 + m / 60) / 12
minute = m / 60

rate = abs(hour - minute)
print(rate * 360 if rate < 0.5 else (1 - rate) * 360)