# DP

N, S = map(int, input().split())
A, B = [0]*N, [0]*N
for i in range(N):
    A[i], B[i] = map(int, input().split())

dp = [[0]*(S+1) for _ in range(N+1)]
dp[0][0] = 1

for i in range(N):
    for j in range(S):
        if dp[i][j]:
            if j + A[i] <= S:
                dp[i+1][j+A[i]] = 1
            if j + B[i] <= S:
                dp[i+1][j+B[i]] = 1

if dp[N][S]:
    print("Yes")
    # DP復元
    ans = []
    now = S
    for i in range(N-1, -1, -1):
        if dp[i][now-A[i]]:
            ans.append("H")
            now -= A[i]
        else:
            ans.append("T")
            now -= B[i]
    print("".join(reversed(ans)))

else:
    print("No")
