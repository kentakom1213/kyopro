#        B - Enlarged Checker Board       
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc250/tasks/abc250_b
# ----------------------------------------

N, A, B = map(int, input().split())

for i in range(N):
    for _ in range(A):
        # タイルの表示
        for j in range(N):
            print(".#"[(i+j)&1] * B, end="")
        print()