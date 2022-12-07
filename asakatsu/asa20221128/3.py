# https://atcoder.jp/contests/abc182/tasks/abc182_d

N = int(input())
A = list(map(int, input().split()))

# 累積和配列
S = [0] * N
S[0] = A[0]
for i in range(1, N):
    S[i] = S[i-1] + A[i]

# Sの累積max
M = [0] * N
M[0] = max(0, S[0])
for i in range(N):
    M[i] = max(M[i-1], S[i-1])

# 解を求める
cur = 0
ans = 0
for i in range(N):
    cur += S[i]
    
    # maxの更新
    if i+1 < N:
        ans = max(
            ans,
            cur + M[i],
        )
    else:
        ans = max(ans, cur)

print(ans)
