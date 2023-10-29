#            D - Distinct Trio            
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc252/tasks/abc252_d

# 緑diffだけど、自力ac厳しい気がする
# ----------------------------------------

N = int(input())
A = list(map(int, input().split()))

MAX_SIZE = 200000

cnt = [0] * (MAX_SIZE + 1)
for i in range(N):
    cnt[A[i]] += 1

for i in range(MAX_SIZE):
    cnt[i+1] += cnt[i]

# cntはi以下の数の個数

ans = 0
for j in range(N):
    ans += cnt[A[j] - 1] * (N - cnt[A[j]])

print(ans)
