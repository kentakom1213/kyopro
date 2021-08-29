# C - 積み重ね
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc006/tasks/arc006_3

# 参考
# https://qiita.com/python_walker/items/d1e2be789f6e7a0851e5


# LIS (LongestIncreasingSubsequence)の問題に帰結できるらしい。
# そろそろDPの勉強もしなければならない。

# WA
# ----------------------------------------

INF = 1e10

# input
N = int(input())
boxes = [int(input()) for _ in range(N)]

# solve
def get_LDS(seq):
    dp = [-INF for _ in range(len(seq))]

    # 線形探索でも問題ないかな？
    for box in boxes:
        ptr = 0
        while dp[ptr] > box:
            ptr += 1
        dp[ptr] = box

    return dp[:dp.index(-INF)] if -INF in dp else dp


counter = 0
while boxes:
    # print(get_LDS(boxes))
    for box in get_LDS(boxes):
        boxes.remove(box)
    counter += 1

print(counter)
