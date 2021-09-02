#             D - Knapsack 1
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/dp/tasks/dp_d
# ----------------------------------------


### 愚直な全探索 O(2^n) ###
# TLE

# input
# N, W = map(int, input().split())
# w, v = [], []
# for _ in range(N):
#     a, b = map(int, input().split())
#     w.append(a)
#     v.append(b)

# # solve
# # 再帰で実装
# def knapsack(i, j):
#     """
#     i : "item[i]"を示す
#     j : ナップサックの残り容量
#     """
#     if i == N:
#         res = 0
#     elif j < w[i]:  # 超えてしまうとき
#         res = knapsack(i+1, j)  # 入れない
#     else:
#         res = max(
#             knapsack(i+1, j),  # 入れない
#             knapsack(i+1, j - w[i]) + v[i] # 入れる
#         )
#     return res

# print(knapsack(0, W))
#########################


###  メモ化再帰  O(nW)  ###
# TLE  これでもダメか

# # input
# N, W = map(int, input().split())
# w, v = [], []
# for _ in range(N):
#     a, b = map(int, input().split())
#     w.append(a)
#     v.append(b)

# # solve
# # メモ化して実装
# DP = {}

# def knapsack_with_memo(i, j):
#     global DP
#     if (i, j) in DP.keys():
#         return DP[(i, j)]
#     """
#     i : "item[i]"を示す
#     j : ナップサックの残り容量
#     """
#     if i == N:
#         res = 0
#     elif j < w[i]:  # 超えてしまうとき
#         res = knapsack_with_memo(i+1, j)  # 入れない
#     else:
#         res = max(
#             knapsack_with_memo(i+1, j),  # 入れない
#             knapsack_with_memo(i+1, j - w[i]) + v[i] # 入れる
#         )

#     DP[(i, j)] = res
#     return res

# print(knapsack_with_memo(0, W))
#########################


###    漸化式  O(nW)   ###

# input
N, W = map(int, input().split())
w, v = [], []
for _ in range(N):
    a, b = map(int, input().split())
    w.append(a)
    v.append(b)

#########################
