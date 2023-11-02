#           D - Not Divisible 
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc170/tasks/abc170_d

# AC
# ----------------------------------------

# 愚直な実装ではO(N^2)

# 方針
# A[i] <= 10^6 であるから、テーブルで管理できる -> エラトステネスのふるい？
# A[i]全てに関してふるいを作成すると O(NlogN)
# 調和級数的計算量という概念らしいです

N = int(input())
A = [int(x) for x in input().split()]
A.sort()

A_MAX = A[-1] + 1

# 重複がある項は無視
dup = set()
pre = None
for n in A:
    if pre == n:
        dup.add(n)
    pre = n

table = [0] * A_MAX
for n in A:
    if n not in dup:
        table[n] = 1

# ふるい落とす
for i in A:
    for j in range(2, (A_MAX-1)//i+1):  # ここのコーナーケースがむずかった
        table[i*j] = 0

print(sum(table))
