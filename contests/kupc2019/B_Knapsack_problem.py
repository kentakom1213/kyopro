#           B - ナップサック問題
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/kupc2019/tasks/kupc2019_b

# 諦める
# ----------------------------------------

def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

# input
n, m, W = map(int, input().split())

weight, value = [], []
for _ in range(n):
    w, v = map(int, input().split())
    weight.append(w)
    value.append(v)

As, Bs = [], []
for _ in range(m):
    a, b = map(int, input().split())
    As.append(a)
    Bs.append(b)


# init DP table
DP = init_array(n+1, W+1)

# solve
def knapsack():
    for i in range(n):
        for sum_w in range(W+1):  # 重さなので+1

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

print(*DP, sep="\n") # test

