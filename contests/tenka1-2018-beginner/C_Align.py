# C - Align
# ----------
# 問題
# https://atcoder.jp/contests/tenka1-2018-beginner/tasks/tenka1_2018_c
# ----------

# <ex1>
# INPUT: 6 8 1 2 3
# SORT : 1 2 3 6 8
# PUSH : 3 6 1 8 2
# COUNT:  3 5 7 6 --> 21

# <ex2>
# INPUT: 3 1 4 1 5 9
# SORT : 1 1 3 4 5 9
# PUSH : 3 5 1 9 1 4
# COUNT:  2 4 8 8 3 --> 25

from collections import deque

N = int(input())
Alist = [int(input()) for _ in range(N)]

# 貪欲
# ソートしてdequeに入れる -> 前後ろから交互にpopして、差が最大になるように追加

Alist.sort()
A = deque(Alist[:])
A_ = deque(Alist[:])

align = deque()
align.append(A.pop())
res = 0

flg = True  # True: 前から, False: 後ろから
while A:
    if flg:
        v = A.popleft()
    else:
        v = A.pop()
    
    diff_l = abs(v - align[0])
    diff_r = abs(v - align[-1])

    if diff_l < diff_r:
        align.append(v)
        res += diff_r
    else:
        align.appendleft(v)
        res += diff_l
    
    flg = not flg

# 後ろからのパターン
align = deque()
align.append(A_.popleft())
res_ = 0

flg = False  # True: 前から, False: 後ろから
while A_:
    if flg:
        v = A_.popleft()
    else:
        v = A_.pop()
    
    diff_l = abs(v - align[0])
    diff_r = abs(v - align[-1])

    if diff_l < diff_r:
        align.append(v)
        res_ += diff_r
    else:
        align.appendleft(v)
        res_ += diff_l


print(max(res, res_))


