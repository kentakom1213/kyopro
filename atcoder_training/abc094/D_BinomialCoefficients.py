#        D - Binomial Coefficients
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc094/tasks/arc095_b

# 細かいミスに気を付ける

# AC
# ----------------------------------------

# nCrを選ぶ
# nは最大値を選べば良い
# nCrが最大となるのは、r = n/2 のときである

n = int(input())
A = list(map(int, input().split()))

ai = max(A)

half = ai / 2
aj = 1e20
for a in A:
    if ai != a and abs(aj - half) > abs(a - half):
        aj = a

print(ai, aj)
