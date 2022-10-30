#               B - Flog 2
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/dp/tasks/dp_b

# 参考
# https://ebisuke33.hatenablog.com/entry/edpc-b

# AC
# ----------------------------------------

# input
N, K = map(int, input().split())
h = list(map(int, input().split()))

# initialize DP table
INF = 1e10
DP = [INF] * N
DP[0] = 0

for i in range(N):
    for k in range(1, K+1):  # 移動なので 1 < k <= K
        if i + k < N:
            # print(f"i:{i} k:{k} DP[{i+k}]:{DP[i+k]}, DP[{i}] + abs(h[{i}] - h[{i+k}]):{DP[i] + abs(h[i] - h[i+k])}")
            DP[i + k] = min(
                DP[i + k],
                DP[i] + abs(h[i] - h[i+k])
            )

print(DP[-1])
