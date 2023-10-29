from bisect import bisect_right
from itertools import product

N, S = map(int, input().split())
A = list(map(int, input().split()))

# 半分全列挙する
left, right = A[:N // 2], A[N // 2:]

# 左側を全列挙
left_all = []
for i, a in enumerate(left):
    tmp = []
    x = a
    while x < S:
        tmp.append(x)
        x *= a
    left_all.append(tmp)

# 左側の組合せをすべて作成
left_all_sorted = sorted(sum(comb) for comb in product(*left_all))

# 右側を全列挙
right_all = []
for i, a in enumerate(right):
    tmp = []
    x = a
    while x < S:
        tmp.append(x)
        x *= a
    right_all.append(tmp)

# 答えを得る
ans = 0
for comb in product(*right_all):
    sum_right = sum(comb)
    ok = bisect_right(left_all_sorted, S - sum_right)
    ans += ok

print(ans)
