#             C - Super Ryuma
# ----------------------------------------
# 問題
# https:#atcoder.jp/contests/abc184/tasks/abc184_c

# WA
# ----------------------------------------

# ちゃんと場合分けできるようになろう

r1, c1 = map(int, input().split())
r2, c2 = map(int, input().split())

def get_num(r1, c1, r2, c2):
    if (r1, c1) == (r2, c2):
        return 0
    elif (r1 - r2)**2 == (c1 - c2)**2 or abs(r1 - r2) + abs(c1 - c2) <= 3:
        return 1
    elif (r1 - r2 + c1 - c2) % 2 == 0 or abs((r2 - r1) - (c2 - c1)) + abs((c2 - c1) - (r2 - r1)) <= 6 or abs(r1 - r2) + abs(c1 - c2) <= 6:
        return 2
    else:
        return 3

print(get_num(r1, c1, r2, c2))