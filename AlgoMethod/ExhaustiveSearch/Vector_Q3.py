#              配列の全探索 3
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/212

# AC
# ----------------------------------------

N = int(input())
A = list(map(int, input().split()))

print(len(list(filter(lambda x: x>0, A))))