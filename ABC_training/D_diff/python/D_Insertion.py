#             D - Insertion
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc064/tasks/abc064_d

# こういう問題好き

# AC
# ----------------------------------------

# N <= 100 であるから、O(n)の操作を繰り返してもそこまで問題ない

# 先頭か末尾に追加すれば良い気がする

from collections import deque

N = int(input())
S = input()

layer = [0] * N
layer[0] = 1 if S[0] == "(" else -1

for i, k in enumerate(S[1:]):
    if k == "(":
        layer[i+1] = layer[i] + 1
    else:
        layer[i+1] = layer[i] - 1

res = deque(S)
for i in range(N):
    # print(layer, end=" -> ")
    if any(x < 0 for x in layer):
        res.appendleft("(")
        layer = [x+1 for x in layer]
    elif layer[-1] > 0:
        res.append(")")
        layer[-1] -= 1
    # print(layer)

print("".join(res))
