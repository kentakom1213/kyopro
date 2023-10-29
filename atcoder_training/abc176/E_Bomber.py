#                E - Bomber               
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc176/tasks/abc176_e
# ----------------------------------------

H, W, M = map(int, input().split())
rows, cols = [0]*H, [0]*W
for i in range(M):
    r, c = map(int, input().split())
    rows[r-1] += 1
    cols[c-1] += 1


