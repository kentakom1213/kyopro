#              C - Ubiquity
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc178/tasks/abc178_c

# 数学的考察が大事、良問

# AC
# ----------------------------------------

N = int(input())

MOD = int(1e9 + 7)

if N == 1:
    print(0)
else:
    all_ = pow(10, N) % MOD
    non_0_or_9 = (-2 * pow(9, N)) % MOD - MOD
    non_0_and_9 = pow(8, N) % MOD

    res = (all_ + non_0_or_9 + non_0_and_9) % MOD
    print(res)
    
