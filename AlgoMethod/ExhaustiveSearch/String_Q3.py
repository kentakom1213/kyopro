#              文字列の全探索 3
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/228

# AC
# ----------------------------------------

S = input()

res = 0

now = " "
for c in S:
    if now == c:
        res += 1
    else:
        now = c
        count = 0

print(res)