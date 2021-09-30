#              文字列の全探索 5
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/229

# AC
# ----------------------------------------

S, T = input(), input()

# print("Yes" if T in S else "No")

# 真面目な実装
i, j = 0, 0
while i < len(S) and j < len(T):
    if S[i] == T[j]:
        i += 1
        j += 1
    else:
        i += 1
        j = 0

print("Yes" if j == len(T) else "No")
