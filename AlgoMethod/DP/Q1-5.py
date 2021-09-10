#            Q1-5. マスの移動 (2)
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/306

# AC
# ----------------------------------------

# N, M = map(int, input().split())
# costs = list(map(int, input().split()))

# DP = [1e10] * N
# DP[0] = 0
# DP[1] = costs[1]

# for i in range(1, N):
#     for M in range(1, i):
#         DP[i] = min(
#             DP[i],
#             DP[i-M] + M * costs[i]
#         )

# print(DP[-1])
### -> WA

### debug
N, M = map(int, input().split())
costs = list(map(int, input().split()))

DP = [1e10] * N
DP[0] = 0
DP[1] = costs[1]

for i in range(1, N):
    for j in range(1, M+1):
        DP[i] = min(
            DP[i],
            DP[i-j] + j * costs[i]
        )

print(DP[-1])