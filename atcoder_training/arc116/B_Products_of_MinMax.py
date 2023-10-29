#         B - Products of Min-Max         
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc116/tasks/arc116_b
# ----------------------------------------

"""comment
- sigmaの計算量を落とす
"""

mod = 998244353

N = int(input())
A = list(map(int, input().split()))

A.sort()

# O(n^2)
# ans = sum(map(lambda x:x*x, A)) % mod

# for i in range(N):
#     for j in range(i+1, N):
#         ans += A[i] * A[j] * pow(2, j-i-1)
#         ans %= mod

# O(n)
ans = sum(map(lambda x:x*x, A)) % mod

# 前計算
Aj = [0] * N
for j in range(1, N):
    Aj[j] = A[j] * pow(2, j-1, mod) % mod

S = [0] * N
S[N-1] = Aj[N-1]
for j in range(N-2, 0, -1):
    S[j] = S[j+1] + Aj[j]

# 和をとる
for i in range(N-1):
    ans += A[i] * pow(2, mod-1-i, mod) * S[i+1]
    ans %= mod

print(ans)
