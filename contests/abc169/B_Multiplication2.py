#           B - Multiplication 2
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc169/tasks/abc169_b

# 実装汚いなあ
# ----------------------------------------

N = int(input())
A = list(map(int, input().split()))

if 0 in A:
    print(0)
    exit()

acc = 1
for a in A:
    acc *= a
    if acc > 10**18:
        print(-1)
        break
else:
    print(acc)