#          D - Flipping Signs
# ----------------------------------------
# 問題 
# https://atcoder.jp/contests/abc125/tasks/abc125_d
# 解説
# https://img.atcoder.jp/abc125/editorial.pdf

# AC (解説)
# ----------------------------------------

# 初期状態で負の数が偶数個 -> 全ての数を正の値にできる
# 　　　　　　　　　奇数個 -> 1つ以外全ての数を正の数にできる

N = int(input())
A = list(map(int, input().split()))

cnt_neg = sum( n < 0 for n in A )
abs_sum = sum( abs(n) for n in A )

if cnt_neg % 2 == 0:
    print(abs_sum)
else:
    abs_min = min( abs(n) for n in A )
    print( abs_sum - 2*abs_min )
