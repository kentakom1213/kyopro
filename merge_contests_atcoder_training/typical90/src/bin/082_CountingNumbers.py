#       082 - Counting Numbers（★3）
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/typical90/tasks/typical90_cd
# ----------------------------------------

MOD = 1000000007

L, R = map(int, input().split())

res = 0
for d in range(19):
    l, r = max(L, 10**d), min(R, int("9"*(d+1)))
    if l <= r:
        sum_ = (l + r) * (r - l + 1) // 2
        res += sum_ * (d + 1)
        res %= MOD

print(res)
