#             C - Super Ryuma
# ----------------------------------------
# 問題
# https:#atcoder.jp/contests/abc184/tasks/abc184_c

# AC (解説)
# ----------------------------------------

# ちゃんと場合分けできるようになろう

# 移動A: |r2 - r1| + |c2 - c1| <= 3 の位置に移動
# 移動B: |r2 - r1| = |c2 - c1|  を見たす位置に移動

# 移動0回: スタート = ゴール
# 移動1回: A: abs(r1 - r2) + abs(c1 - c2) <= 3:
#         B: (r1 - r2)**2 == (c1 - c2)**2
# 移動2回: A + A: abs(r1 - r2) + abs(c1 - c2) <= 6
#         A + B: abs((r2 - r1) - (c2 - c1)) + abs((c2 - c1) - (r2 - r1)) <= 6
#         B + B: (r1 - r2 + c1 - c2) % 2 == 0
# 移動3回: その他


r1, c1 = map(int, input().split())
r2, c2 = map(int, input().split())

def get_num(r1, c1, r2, c2):
    if (r1, c1) == (r2, c2):
        return 0
    elif (r1 - r2)**2 == (c1 - c2)**2 or abs(r1 - r2) + abs(c1 - c2) <= 3:
        return 1
    elif (r1 - r2 + c1 - c2) % 2 == 0 or abs((r1 + c1) - (r2 + c2)) <= 3 or abs((r1 - c1) - (r2 - c2)) <= 3 or abs(r1 - r2) + abs(c1 - c2) <= 6:
        return 2
    else:
        return 3

print(get_num(r1, c1, r2, c2))

# r1, c1 = 15, 15
# for c2 in range(30):

#     l = [(1 if abs((r1 + c1) - (r2 + c2)) <= 3 or abs((r1 - c1) - (r2 - c2)) <= 3 else 0) for r2 in range(30)]
#     print(l)
