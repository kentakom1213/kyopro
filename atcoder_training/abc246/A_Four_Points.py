#             A - Four Points             
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc246/tasks/abc246_a
# ----------------------------------------

X, Y = [], []
for _ in range(3):
    x, y = map(int, input().split())
    if x in X:
        X.remove(x)
    else:
        X.append(x)
    if y in Y:
        Y.remove(y)
    else:
        Y.append(y)
    

print(X[0], Y[0])
