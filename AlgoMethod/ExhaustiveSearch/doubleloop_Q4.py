#              文字列の全探索 4
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/236

# AC
# ----------------------------------------

S = input()
print( len(set(filter(str.islower, S))) )