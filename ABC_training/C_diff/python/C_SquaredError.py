#           C - Squered Error
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc194/tasks/abc194_c

# 参考
# https://atcoder.jp/contests/abc194/editorial/826

# AC (解説)
# ----------------------------------------

# 二乗和誤差に関する問題
# 方針
# 全て計算すると、 O(N^2) で間に合わない
# |Ai| <= 200 を利用

N = int(input())
A = list(map(int, input().split()))

# counter = [A.count(i) for i in range(-200, 201)]  # O(N) -> TLE
# 400 * 3 * 10^5 ~= 10^8 で通らないか

A.sort()

counter = [0] * 401
now = A[0]
for i in range(N):
    if A[i] > now: now = A[i]
    counter[now + 200] += 1


# O(1)
res = 0
for i in range(1, 401):
    for j in range(i):
        error = (i - j) ** 2
        res += error * counter[i] * counter[j]

print(res)