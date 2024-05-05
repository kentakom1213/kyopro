#          B - Electric Board
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc119/tasks/arc119_b
# ----------------------------------------

N = int(input())
S, T = input(), input()

if sum(map(int, S)) != sum(map(int, T)):
    print(-1)
    exit()

cnt = 0
for s, t in zip(S, T):
    cnt += s != t

print(cnt)
