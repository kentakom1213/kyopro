#            B - Many Oranges
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc195/tasks/abc195_b

# 参考
# https://blog.hamayanhamayan.com/entry/2021/03/14/001917

# B問題最強難度では?

# AC (解説)
# ----------------------------------------

# 全探索してみよう

A, B, W = map(int, input().split())
W *= 1000

min_mikan, max_mikan = 1e10, -1

for choose in range(1, W+1):
    L = choose * A
    R = choose * B
    if L <= W <= R:
        min_mikan = min(min_mikan, choose)
        max_mikan = max(max_mikan, choose)

if min_mikan == 1e10:
    print("UNSATISFIABLE")
else:
    print(min_mikan, max_mikan)