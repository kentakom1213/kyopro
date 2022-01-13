# A - STring
# -----------
# 問題
# https://atcoder.jp/contests/agc005/tasks/agc005_a

# AC
# -----------

# かっこのパースと同じ原理
# stackで処理

S = input()

stack = []
for s in S:
    if stack:
        if stack[-1] == "S" and s == "T":
            stack.pop()
        else:
            stack.append(s)
    else:
        stack.append(s)

print(len(stack))

