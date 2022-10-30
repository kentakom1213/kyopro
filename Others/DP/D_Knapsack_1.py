#             D - Knapsack 1
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/dp/tasks/dp_d

# 難しい、要練習

# AC
# ----------------------------------------


### 愚直な全探索 O(2^n) ###
# TLE

# input
N, W = map(int, input().split())
w, v = [], []
for _ in range(N):
    a, b = map(int, input().split())
    w.append(a)
    v.append(b)

# solve
# 再帰で実装
def knapsack(i, j):
    """
    i : "item[i]"を示す
    j : ナップサックの残り容量
    """
    if i == N:
        res = 0
    elif j < w[i]:  # 超えてしまうとき
        res = knapsack(i+1, j)  # 入れない
    else:
        res = max(
            knapsack(i+1, j),  # 入れない
            knapsack(i+1, j - w[i]) + v[i] # 入れる
        )
    return res

print(knapsack(0, W))
#########################


###  メモ化再帰  O(NW)  ###
# TLE  これでもダメか

# input
N, W = map(int, input().split())
w, v = [], []
for _ in range(N):
    a, b = map(int, input().split())
    w.append(a)
    v.append(b)

# solve
# メモ化して実装
memo = {}

def knapsack_with_memo(i, j):
    global memo
    if (i, j) in memo.keys():
        return memo[(i, j)]
    """
    i : "item[i]"を示す
    j : ナップサックの残り容量
    """
    if i == N:
        res = 0
    elif j < w[i]:  # 超えてしまうとき
        res = knapsack_with_memo(i+1, j)  # 入れない
    else:
        res = max(
            knapsack_with_memo(i+1, j),  # 入れない
            knapsack_with_memo(i+1, j - w[i]) + v[i] # 入れる
        )

    memo[(i, j)] = res
    return res

print(knapsack_with_memo(0, W))
#########################


###    漸化式  O(NW)   ###
# 参考
# https://qiita.com/drken/items/dc53c683d6de8aeacf5a

# AC

def exprint(l): print(*l, sep="\n")
def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

# input
N, W = map(int, input().split())
w, v = [], []
for _ in range(N):
    a, b = map(int, input().split())
    w.append(a)
    v.append(b)

# init DP table
DP = init_array(N+1, W+1, 0)  # dp[ i ][ sum_w ] := i-1 番目までの品物から重さが sum_w を超えないように選んだときの、価値の総和の最大値

def knapsack_DP():
    for i in range(N):
        for sum_w in range(W+1):

            # i番目の品物を選ぶ場合
            if sum_w - w[i] >= 0:
                DP[i+1][sum_w] = max(
                    DP[i+1][sum_w],
                    DP[i][sum_w - w[i]] + v[i]
                )

            # 選ばない場合
            DP[i+1][sum_w] = max(
                DP[i+1][sum_w],
                DP[i][sum_w]
            )

knapsack_DP()

# exprint(DP) # test
print(DP[N][W])
#########################
