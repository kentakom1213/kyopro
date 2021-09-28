#              配列の全探索 9
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/214

# AC
# ----------------------------------------

N = int(input())
A = list(map(int, input().split()))

res = [0] * 9
for a in A:
    res[a-1] += 1

print(*res, sep="\n")