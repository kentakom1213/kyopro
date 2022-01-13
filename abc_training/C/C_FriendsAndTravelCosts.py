#      C - Friends and Travel costs
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc203/tasks/abc203_c

# AC (解説)
# ----------------------------------------

# 方針
# DP

# def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

# N, K = map(int, input().split())
# AB = [tuple(map(int, input().split())) for _ in range(N)]



# DP = [0] * (N + 1)
# DP[0] = K

# # k -> i の移動
# for i in range(1, N+1):
#     for k in range(i):
#         print(f"DP[{k}] - (A[{i-1}] - {k}) + B[{i-1}] = {DP[k]} - ({A[i-1]} - {k}) + {B[i-1]} = {DP[k] - (A[i-1] - k) + B[i-1]}")
#         if DP[k] > A[i-1] - k:
#             DP[i] = max(
#                 DP[i],
#                 DP[k] - (A[i-1] - k) + B[i-1]
#             )

# print(DP)

# travel = [A[i] + DP[i+1] for i in range(N)]

# print(travel)
## -> ???

# 解説
# 全て通るから色々考えなくていい。手順通りに動く

N, K = map(int, input().split())
AB = sorted( tuple(map(int, input().split())) for _ in range(N) )

for i in range(N):
    if AB[i][0] > K:
        break
    K += AB[i][1]

print(K)