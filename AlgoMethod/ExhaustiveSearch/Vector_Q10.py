#              配列の全探索 10
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/219

# AC
# ----------------------------------------

N = int(input())
A = list(map(int, input().split()))

res = [0] * 9
for a in A:
    res[a-1] += 1

max, max_i = 0, 0
for i, a in enumerate(res):
    if max < a:
        max = a
        max_i = i

print(max_i + 1)