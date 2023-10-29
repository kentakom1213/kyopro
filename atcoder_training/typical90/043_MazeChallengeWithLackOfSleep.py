# 043 - Maze Challenge with Lack of Sleep（★4） 
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/typical90/tasks/typical90_aq
# ----------------------------------------

# 拡張bfsを使う

# 1. 向いている方向に進む　 : コスト0
# 2. 向いている方向を変える : コスト1

H, W = map(int, input().split())
S = rs, cs = [int(v)-1 for v in input().split()]
T = rt, ct = [int(v)-1 for v in input().split()]
field = [list(map(int, input().split())) for _ in range(H)]

def dijkstra(s):
    
