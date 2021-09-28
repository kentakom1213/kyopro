#              配列の全探索 2
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/210

# AC
# ----------------------------------------

N, V = map(int, input().split())
A = list(map(int, input().split()))

counter = 0
for i in A:
    if i == V:
        counter += 1

print(counter)