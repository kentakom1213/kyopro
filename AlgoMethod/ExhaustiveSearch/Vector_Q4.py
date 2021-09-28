#              配列の全探索 4
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/216

# AC
# ----------------------------------------

N, V = map(int, input().split())
A = list(map(int, input().split()))

res = -1
for i, a in enumerate(A):
    if a == V:
        res = i

print(res)