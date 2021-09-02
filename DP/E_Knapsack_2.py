# E - Knapsack 2
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/dp/tasks/dp_e

# 参考
# https://qiita.com/drken/items/dc53c683d6de8aeacf5a

# 方針
# Knapsack1とは制約だけ異なる
# ↓ D問題の方針
# dp[ i ][ sum_w ] :=
#   i-1 番目までの品物から重さが sum_w を超えないように選んだときの、"価値の総和の最大値"
# ↓ E問題の方針
# dp[ i ][ sum_v ] :=
#   i-1 番目までの品物から価値が sum_v となるように選んだときの、"重さの総和の最小値"

# DPむずくない？？

# AC
# ----------------------------------------

def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

# input  D問題と共通
N, W = map(int, input().split())
w, v = [], []
for _ in range(N):
    a, b = map(int, input().split())
    w.append(a)
    v.append(b)
MAX_V = sum(v) + 1

# init DPtable
INF = 1e10
DP = init_array(N+1, MAX_V, INF)
DP[0][0] = 0

# solve
def knapsack():
    for i in range(N):
        for sum_v in range(MAX_V):

            DP[i+1][sum_v] = min(
                DP[i+1][sum_v],
                DP[i][sum_v - v[i]] + w[i],  # i番目を選ぶ場合
                DP[i][sum_v]                 # i番目を選ばない場合
            )

    # best value
    for j in range(MAX_V):
        if DP[N][j] <= W:
            res = j

    return res


print(knapsack())
