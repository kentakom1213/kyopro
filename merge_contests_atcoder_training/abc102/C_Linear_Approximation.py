#         C - Linear Approximation        
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc102/tasks/arc100_a
# ----------------------------------------

N = int(input())
A = [int(v) for v in input().split()]

# iを引く
A = [a - i for i, a in enumerate(A)]

# 0以上に正規化
mini = min(A)
A = [a - mini for a in A]

# ソート
A.sort()

# 累積和をとる
S = [0] * (N + 1)
for i in range(N):
    S[i + 1] = S[i] + A[i]

# i番目を基準にしたときの値の最小値を取得
ans = 1e20

for i in range(N):
    l = A[i] * i - S[i]
    r = (S[N] - S[i]) - A[i] * (N - i)
    ans = min(ans, l + r)

print(ans)
