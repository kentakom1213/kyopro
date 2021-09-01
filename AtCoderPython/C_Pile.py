# C - 積み重ね
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc006/tasks/arc006_3

# 参考
# https://qiita.com/python_walker/items/d1e2be789f6e7a0851e5
# https://drken1215.hatenablog.com/entry/2020/12/25/185948
# [1] https://drken1215.hatenablog.com/entry/2020/12/25/184700

# LIS (LongestIncreasingSubsequence)の問題に帰結できるらしい。
# そろそろDPの勉強もしなければならない。

# AC
# ----------------------------------------

from bisect import bisect_left
INF = 1e10
def mapl(func, iter): return list(map(func, iter))
def filterl(func, iter): return list(filter(func, iter))

# input
N = int(input())
boxes = [int(input()) for _ in range(N)]

# solve
def LIS(seq):
    dp = [INF] * len(seq)
    for val in seq:
        dp[bisect_left(dp, val)] = val

    return dp

print( len( filterl(lambda x: x != INF, LIS(boxes)) ) )


### なぜLISが山の数になるのか？

# def LIS_with_history(seq):
#     history = [[] for _ in range(len(seq))]
#     dp = [INF] * len(seq)
#     for val in seq:
#         i = bisect_left(dp, val)
#         history[i].append(val)  # 過程を保存
#         dp[i] = val

#     return history

# print(LIS_with_history(boxes))

# In:
# 15
# 3
# 1
# 4
# 1
# 5
# 9
# 2
# 6
# 5
# 3
# 5
# 8
# 9
# 7
# 9
# 6

# Out:
# [[3, 1, 1], [4, 2], [5, 5, 3], [9, 6, 5], [8, 7], [9, 9], [], [], [], [], [], [], [], [], []]

# LISを求める過程を可視化すると、Outのようにdpテーブルが書き変わっていくのがわかる。
# このとき、書き変わっていく過程で発生するリストは、この問題でいう'山'に一致する。

# 参考[1]より抜粋
# 要素 Ai を考えるとき、
# - すでにある各山のてっぺんの値のうち、Ai 未満となっている範囲で最大の要素のところに Ai を重ねる
# - そのような山が存在しない場合は新たに Ai のみからなる山を作る

# これにより、LISを求める過程と、山を作る過程は互いに等しいことだとわかる。
