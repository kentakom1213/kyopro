#         B - Takahashi's Failure         
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc252/tasks/abc252_b
# ----------------------------------------

N, K = map(int, input().split())
A = list(map(int, input().split()))
B = list(map(int, input().split()))

oisii = max(A)
print("Yes" if any(A[i]==oisii and i+1 in B for i in range(N)) else "No")
