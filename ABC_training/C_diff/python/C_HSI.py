#                  C - HSI
# ----------------------------------------
# 問題
# https:#atcoder.jp/contests/abc078/tasks/arc085_a

# 良問

# AC
# ----------------------------------------

N, M = map(int, input().split())

print((1900 * M + 100 * (N - M)) * 2 ** M)