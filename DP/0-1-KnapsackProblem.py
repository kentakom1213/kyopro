#          0-1 Knapsack Problem
# ----------------------------------------
# 問題
# https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_B&lang=jp

# 参考
# https://oretano.com/dynamic-programming
# DPがよく分かっていないので、その練習として。

# むずいのでとりあえずパス
# ----------------------------------------

def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

# input
N, W = map(int, input().split())
value, weight = [], []
for _ in range(N):
    v, w = map(int, input().split())
    value.append(v)
    weight.append(w)

# solve
def knapsack(N, W):
    DP = init_array(N+1, W+1)

    for i in range(N):
        for sum_w in range(W+1):

            # この荷物を追加する場合
            if sum_w - weight[i] >= 0:
                DP[i+1][sum_w] = max(
                    DP[i+1][sum_w],
                    DP[i+1][sum_w - weight[i]] + value[i]
                )

            # 荷物を追加しない場合
            DP[i+1][sum_w] = max(
                DP[i+1][sum_w],
                DP[i][sum_w]
            )

        print(*DP, sep="\n")  # test
        print()

    # print(*DP, sep="\n")  # test
    print(DP[-1][-1])


if __name__ == "__main__":
    knapsack(N, W)
