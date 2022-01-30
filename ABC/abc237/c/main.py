S = input()

# 前のaをカウント
top_a = 0
for c in S:
    if c == "a":
        top_a += 1
    else:
        break

# 後ろのaをカウント
back_a = 0
for c in reversed(S):
    if c == "a":
        back_a += 1
    else:
        break

if top_a == back_a:
    isOK = S == S[::-1]
elif top_a < back_a:
    S_ = "a" * (back_a - top_a) + S
    isOK = S_ == S_[::-1]
else:
    isOK = False

print("Yes" if isOK else "No")
