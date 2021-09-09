#             Q2-1. 表と数字 (1)
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/324
# ----------------------------------------

A = list(map(int, input().split()))

print( sum(A[i]*(i+1) for i in range(4)) )