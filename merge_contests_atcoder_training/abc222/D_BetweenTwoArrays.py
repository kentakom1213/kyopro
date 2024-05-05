

# N = int(input())
# A = list(map(int, input().split()))
# B = list(map(int, input().split()))

# res = [1] * N
# for i in range(N):
#     a, b = A[i], B[i]
#     for j in range(a, b+1):
#         if i == 0:
#             res = b-a+1
#         else:
#             res[i] += 



### コンテスト後
# AC

def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

MOD = 998244353

N = int(input())
A = list(map(int, input().split()))
B = list(map(int, input().split()))

MAX = 3001
dp = init_array(N, MAX)
S = init_array(N, MAX)

for i in range(N):
    for j in range(MAX):
        if A[i] <= j <= B[i]:
            if i == 0:
                dp[i][j] = 1
            else:
                dp[i][j] = S[i-1][j]
        

        S[i][j] = (S[i][j-1] + dp[i][j]) % MOD


print(S[N-1][-1])