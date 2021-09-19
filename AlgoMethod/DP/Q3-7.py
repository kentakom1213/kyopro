#         Q3-7. ボールと 2 つの箱
# ----------------------------------------
# 問題
# https://algo-method.com/tasks/353

# DPコンプリート!!

# AC
# ----------------------------------------

def exprint(x): print(*x, sep="\n")
def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

N = int(input())
W = list(map(int, input().split()))
sum_half = sum(W) // 2

dp = init_array(N+1, sum_half+1, False)
dp[0][0] = True

for i in range(N):
    for j in range(sum_half):
        # 到達可能
        if dp[i][j]:
            # 加える場合
            if j + W[i] <= sum_half:
                dp[i+1][j+W[i]] = True
            
            dp[i+1][j] = True

# exprint(dp)

max_half = 0
for i in range(sum_half+1):
    if dp[-1][i]:
        max_half = i

print(sum(W) - 2 * max_half)


# dp[i][j] := N個までのボールを使って重さjを表現できるか
# 最終的にjを sum_half の範囲で最大化すればよい