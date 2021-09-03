#           B - ナップサック問題
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/kupc2019/tasks/kupc2019_b
# ----------------------------------------

def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

# input
n, m, W = map(int, input().split())

weight, value = [], []
for _ in range(m):
    w, v = map(int, input().split())
    weight.append(w)
    value.append(v)


# init DP table
DP = init_array(m+1, W+1)

# solve
def knapsack():
    for i in range(m):
        for sum_w in range(W):

            # 品物を追加するとき
            if sum_w - weight[i] >= 0:
                DP[i+1][sum_w] = max(
                    DP[i+1][sum_w],
                    DP[i][sum_w - weight[i]] + value[i],
                )
                    
            # 追加しないとき
            DP[i+1][sum_w] = max(
                DP[i+1][sum_w],
                DP[i][sum_w]
            )


knapsack()

print(DP) # test
                    
