#       069 - Colorful Blocks 2（★3）
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/typical90/tasks/typical90_bq

# 参考
# https://atcoder.jp/contests/typical90/editorial/2056

# ----------------------------------------

# 普通にDPで解く

N, K = map(int, input().split())

color = [0] * N  # 使える色の種類
dp = [0] * N  # dp[i] := i番目までの要素を使って塗れる組み合わせ

color[0] = K
dp[0] = K

for i in range(1, N):
    color[i] = color[i-1] - 1 