#       069 - Colorful Blocks 2（★3）
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/typical90/tasks/typical90_bq
# 参考
# https://atcoder.jp/contests/typical90/editorial/2056

# AC (解説)
# ----------------------------------------

MOD = 1000000007
N, K = map(int, input().split())
ans = K if N == 1 else K * (K-1) * pow(K-2, N-2, MOD) % MOD
print(ans)