#              B - Count 1's              
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc137/tasks/arc137_b
# ----------------------------------------

# 方針
# --------------------
# 区間 [i, j) に含まれる0の個数を f(i,j)
# 区間 [i, j) に含まれる1の個数を g(i,j) と定義する。
# S(i,j) := f(i,j) - g(i,j) の最大値、最小値を求めることで解の取りうる値の範囲を求める。
# ↓
# 範囲内の数は全て取りうる値となる
# --------------------

# solve
# --------------------

N = int(input())
A = map(int, input().split())

now = 0          # now := g(0, i) - f(0, i)
x = y = 0        # x := 
mini = maxi = 0
for a in A:
    if a == 0:
        now -= 1
    else:
        now += 1

    x = min(x, now - maxi)
    y = max(y, now - mini)
    if mini > now: mini = now
    if maxi < now: maxi = now

print(y - x + 1)
