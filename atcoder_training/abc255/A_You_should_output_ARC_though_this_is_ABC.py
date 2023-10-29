# A - You should output ARC, though this is ABC.
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc255/tasks/abc255_a
# ----------------------------------------

R, C = map(int, input().split())
print([
    list(map(int, input().split())),
    list(map(int, input().split()))
][R-1][C-1])


