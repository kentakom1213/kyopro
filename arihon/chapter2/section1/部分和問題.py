"""
# 部分和問題
- P34

## 制約
- $1 \le n \le 20$

n<=20 だからbit全探索が使える
"""

N, K = map(int, input().split())
A = list(map(int, input().split()))

for i in range(1 << N):
    s = 0
    for j in range(N):
        if (i >> j) & 1:
            s += A[j]
    if s == K:
        print("Yes")
        exit()

print("No")
