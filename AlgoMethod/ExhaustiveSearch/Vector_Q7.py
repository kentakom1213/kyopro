#              配列の全探索 7
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/211

# AC
# ----------------------------------------

N = int(input())
A = list(map(int, input().split()))

max, max_i = 0, 0
for i, a in enumerate(A):
    if max < a:
        max, max_i = a, i

print(max_i)