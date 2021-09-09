#            Q1-2. マスの移動 (1)
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/303

# AC
# ----------------------------------------

N = int(input())
costs = list(map(int, input().split()))

INF = 1e10
DP = [INF] * N
DP[0] = 0
DP[1] = costs[1]

for i in range(2, N):
    DP[i] = min(
        DP[i-1] + costs[i],
        DP[i-2] + 2 * costs[i]
    )

print(DP[-1])