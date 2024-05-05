#        070 - Plant Planning（★4）
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/typical90/tasks/typical90_br
# ----------------------------------------

N = int(input())
plants = [tuple(map(int, input().split())) for _ in range(N)]

X, Y = zip(*plants)
X, Y = sorted(X), sorted(Y)

# - 左側の点が多いとき、左に動くのがいい
# - 右側の点が多いとき、右に動くのがいい
# よって、中央値の点を選ぶと、絶対値の和を最小化できる

cX, cY = X[N//2], Y[N//2]

ans = sum(abs(x-cX)+abs(y-cY) for x, y in plants)
print(ans)
