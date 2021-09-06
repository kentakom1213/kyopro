#            C - 数を3つ選ぶマン
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc028/tasks/abc028_c

# シンプルイズベスト

# AC
# ----------------------------------------

n = list(map(int, input().split()))

min2 = min(
    n[0] + n[3],
    n[1] + n[2]
)

print(sum(n) - min2)
