#          Q3-4. ナップサック問題
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/342

# AC
# ----------------------------------------

def exprint(x): print(*x, sep="\n")
def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

N, W = map(int, input().split())
weights = list(map(int, input().split()))
values = list(map(int, input().split()))

dp = init_array(N+1, W+1)

for i in range(N):
    for sum_w in range(W+1):
        # 加える場合
        if sum_w + weights[i] < W+1:
            dp[i+1][sum_w + weights[i]] = max(
                dp[i][sum_w + weights[i]],
                dp[i][sum_w] + values[i]
            )
        # 加えない場合
        dp[i+1][sum_w] = max(
            dp[i][sum_w],
            dp[i+1][sum_w]
        )

print(dp[-1][-1])
