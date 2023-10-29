#             A - 点数変換
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc043/tasks/arc043_a
# ----------------------------------------

N, A, B = map(int, input().split())
scores = [int(input()) for _ in range(N)]

diff = max(scores) - min(scores)

if diff == 0:
    print(-1)
    exit()

P = B / diff

aveS = sum(scores) / N
Q = A - P * aveS

print(P, Q)
