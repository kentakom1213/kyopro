#         D - おいしいたこ焼きの焼き方
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc005/tasks/abc005_4

# 参考
# https://ymduu.hatenablog.com/entry/2018/05/08/215119

# 二次元累積和、テーブル生成
# ----------------------------------------

def mapl(func, iter): return list(map(func, iter))

# input
N = int(input())
D = [mapl(int, input().split()) for _ in range(N)]

Q = int(input())
P = [int(input()) for _ in range(Q)]
