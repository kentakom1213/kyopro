#               配列の全探索 1
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/209

# AC
# ----------------------------------------

N, V = map(int, input().split())
A = list(map(int, input().split()))

print( "Yes" if V in A else "No" )