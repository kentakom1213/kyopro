# https://atcoder.jp/contests/abc209/tasks/abc209_b

N, X = map(int, input().split())
A = list(map(int, input().split()))

s = 0
for i in range(N):
    s += A[i] - i % 2

print("Yes" if s <= X else "No")
