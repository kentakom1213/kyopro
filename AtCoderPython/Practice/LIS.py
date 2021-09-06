#                 LISの練習
# ----------------------------------------
# 問題
# 与えられた数列に対し、最大増加部分列を求める
# 参考
# https://qiita.com/python_walker/items/d1e2be789f6e7a0851e5
# https://aotamasaki.hatenablog.com/entry/bisect_for_reversed_list
# ----------------------------------------



# input
N = int(input())
seq = list(map(int, input().split()))

# solve
INF = 1e20
dp = [INF for _ in range(N)]

for a in seq:
    # print(dp)
    ptr = 0
    while dp[ptr] < a:
        ptr += 1

    dp[ptr] = a

print(dp)

# ^ ただし線形探索（計算量は O(N^2)）

### 二分探索での実装（計算量は O(NlogN)）
import bisect

LIS = [seq[0]]
for i in range(N):
    if seq[i] > LIS[-1]:
        LIS.append(seq[i])
    else:
        LIS[bisect.bisect_left(LIS, seq[i])] = seq[i]

print(LIS)


### 最大減少列を探す
def bisect_left_reverse(a, x):
    '''
    reverseにソートされたlist aに対してxを挿入できるidxを返す。
    xが存在する場合には一番左側のidxとなる。
    '''
    if a[0] <= x:
        return 0
    if x < a[-1]:
        return len(a)
    # 二分探索
    ok = len(a) - 1
    ng = 0
    while (abs(ok - ng) > 1):
        mid = (ok + ng) // 2
        if a[mid] <= x:
            ok = mid
        else:
            ng = mid
    return ok

LIS = [seq[0]]
for i in range(N):
    if seq[i] < LIS[-1]:
        LIS.append(seq[i])
    else:
        LIS[bisect_left_reverse(LIS, seq[i])] = seq[i]

print(LIS)
