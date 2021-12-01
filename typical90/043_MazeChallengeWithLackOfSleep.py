# 043 - Maze Challenge with Lack of Sleep（★4） 
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/typical90/tasks/typical90_aq
# ----------------------------------------

# bfsで曲がり角に関してDPを行う

H, W = map(int, input().split())
S = (rs, cs) = [int(i)-1 for i in input().split()]
T = (tt, ct) = [int(i)-1 for i in input().split()]
field = [input() for _ in range(H)]

is_visited = [[1e9]*W for _ in range(H)]
