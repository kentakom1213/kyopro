#            C - Gap Existence            
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc296/tasks/abc296_c
# ----------------------------------------

N, X = map(int, input().split())
A = list(map(int, input().split()))

st = set(A)

for a in A:
    if a+X in st or a-X in st:
        print("Yes")
        exit()

print("No")