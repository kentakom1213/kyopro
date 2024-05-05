#             C - Sugar Water
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc074/tasks/arc083_a
# ----------------------------------------

MAX = 3030

a, b, c, d, e, f = map(int, input().split())

# 作れる水の量
water = [0] * MAX
water[0] = True
for i in range(MAX):
    if water[i]:
        if i + 100 * a < MAX:
            water[i + 100 * a] = True
        if i + 100 * b < MAX:
            water[i + 100 * b] = True

water = [i for i in range(MAX) if water[i]]

sugar = [0] * MAX
sugar[0] = True
for i in range(MAX):
    if sugar[i]:
        if i + c < MAX:
            sugar[i + c] = True
        if i + d < MAX:
            sugar[i + d] = True

sugar = [i for i in range(MAX) if sugar[i]]

# 全探索
ans_w, ans_s = 0, 0
for w in water:
    for s in sugar:
        # 実行可能か
        if w + s <= f and 100 * s <= w * e:
            # s / (w + s) >= ans_s / (ans_w + ans_s)
            if s * (ans_w + ans_s) >= ans_s * (s + w):
                ans_w = w
                ans_s = s

print(ans_w + ans_s, ans_s)
