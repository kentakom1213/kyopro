#         Q3-5. 部分和問題 (応用 1)
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/350

# AC (解説)
# ----------------------------------------

# dp[i][j] := i個目までの荷物を使って重さの合計をjにできるとき、荷物の最小の数


def exprint(x): print(*x, sep="\n")
def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

N, W = map(int, input().split())
weights = list(map(int, input().split()))

INF = 1e9
dp = init_array(N+1, W+1, INF)
dp[0][0] = 0

for i in range(N):
    for sum_w in range(W+1):
        # 入れる場合
        if sum_w + weights[i] < W+1:
            dp[i+1][sum_w + weights[i]] = min(
                dp[i+1][sum_w + weights[i]],
                dp[i][sum_w] + 1
            )
        
        # 入れない場合
        dp[i+1][sum_w] = min(
            dp[i+1][sum_w],
            dp[i][sum_w]
        )

# exprint(dp)
print(dp[-1][-1] if dp[-1][-1] < 1e9 else -1)