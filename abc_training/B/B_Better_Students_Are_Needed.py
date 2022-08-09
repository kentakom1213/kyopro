#     B - Better Students Are Needed!     
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc260/tasks/abc260_b
# ----------------------------------------

N, X, Y, Z = map(int, input().split())
A = list(map(int, input().split()))
B = list(map(int, input().split()))

passed = set()

# 数学でソート
math = sorted(range(N), key=lambda i: (-A[i], i))
for i in range(X):
    passed.add(math[i])

# 英語でソート
eng = sorted(range(N), key=lambda i: (i in passed, -B[i], i))
for i in range(Y):
    passed.add(eng[i])

# 総合得点でソート
summ = sorted(range(N), key=lambda i: (i in passed, -(A[i]+B[i]), i))
for i in range(Z):
    passed.add(summ[i])

# 順に表示
for i in sorted(passed):
    print(i+1)
