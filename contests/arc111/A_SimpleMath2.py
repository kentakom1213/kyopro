#          A - Simple Math 2
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc111/tasks/arc111_a
# ----------------------------------------

# floor(10^N / M) = floor(10^N % M^2 / M)

N, M = map(int, input().split())

x = pow(10, N, M**2) // M
print(x % M)
