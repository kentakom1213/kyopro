#                 H - LIS
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/chokudai_S001/tasks/chokudai_S001_h

# 参考
# https://eromog.hatenablog.com/entry/2019/07/26/025035

# なぜかWA
# ----------------------------------------

from bisect import bisect_left
INF = 1e10

N = int(input())
sequence = list(map(int, input().split()))

def getLIS():
    DP = [INF] * (N + 1)

    for v in sequence:
        DP[bisect_left(DP, v)] = v

    # print(DP)  # test

    subseq = list(filter(lambda i: i<1e5, DP))
    return len(subseq)
    
print(getLIS())
