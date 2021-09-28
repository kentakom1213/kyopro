#              配列の全探索 6
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/213

# AC
# ----------------------------------------

N = int(input())
A = list(map(int, input().split()))

max = - 1e10
for a in A:
    if a > max:
        max = a

print(max)