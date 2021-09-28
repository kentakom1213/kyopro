#              配列の全探索 5
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/215

# AC
# ----------------------------------------

N = int(input())
A = list(map(int, input().split()))

res = 0
for i in range(1, len(A)):
    if A[i] > A[i-1]:
        res += 1

print(res)