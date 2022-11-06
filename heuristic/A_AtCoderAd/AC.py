#             A - AtCoder Ad
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/ahc001/tasks/ahc001_a

# 参考
# https://www.terry-u16.net/entry/ahc001-how-to
# ----------------------------------------


# AC (ひどいコードだけど) -> 0点
# n = int(input())
# rects = [tuple(map(int, input().split())) for _ in range(n)]

# x, y = 0, 0

# while n:
#     print(x, y, x+1, y+1)
#     x = (x + 1) % 10000
#     y = (y + 1) // 10000
#     n -= 1

# 正の点数を目指す
n = int(input())
rects = [tuple(map(int, input().split())) for _ in range(n)]

for x, y, r in rects:
    print(x, y, x+1, y+1)