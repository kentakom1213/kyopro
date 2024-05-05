# D - ナップサック問題
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc032/tasks/abc032_d

# 半分前列挙？？
# パス
# ----------------------------------------

def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

def knapsack(N, W, values, weights):
    DP = init_array(N+1, W+1)



if __name__ == "__main__":
    N, W = map(int, input().split())
    values, weights = [], []
    for _ in range(N):
        v, w = map(int, input().split())
        values.append(v)
        weights.append(w)

    knapsack(N, W, values, weights)