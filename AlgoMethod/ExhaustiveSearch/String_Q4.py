#              文字列の全探索 4
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/230

# AC
# ----------------------------------------

N = int(input())
S, T = input(), input()

res = 0
for s, t in zip(S, T):
    if s != t:
        res += 1

print(res)