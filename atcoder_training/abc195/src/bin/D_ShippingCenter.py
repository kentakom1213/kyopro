# D - Shipping Center
# --------------------
# 問題
# https://atcoder.jp/contests/abc195/tasks/abc195_d

# 解説
# https://atcoder.jp/contests/abc195/editorial/846

# AC (解説)
# --------------------

# あらかじめ荷物を価値順にソート
# 範囲を抽出 -> 貪欲
# O(NlogN Q)
# N <= 50, Q <= 50, W <= 10^6

# クエリごとに独立に問題をとく

def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

N, M, Q = map(int, input().split())

items = [tuple(map(int, input().split())) for _ in range(N)]
items.sort(key=lambda x: (x[1], x[0]), reverse=True)  # 価値: 降順, 重量: 降順

boxes = list(map(int, input().split()))

for i in range(Q):
    l, r = map(int, input().split())
    enable = sorted(boxes[:l-1] + boxes[r:])
    total = len(enable)

    # 貪欲に荷物を当てはめる
    is_shipped = [0] * N
    max_value = 0
    for box in enable:
        for i, (w, v) in enumerate(items):
            if is_shipped[i]:
                continue
            if w <= box:
                max_value += v
                is_shipped[i] = 1
                break

    print(max_value)

