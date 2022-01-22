#               A - A*B*C
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc113/tasks/arc113_a

# AC (解説)
# ----------------------------------------

# 解説
# 調和級数

ceil = lambda x: int(-(-x//1))

K = int(input())

cnt = 0
for a in range(1, K+1):
    for b in range(1, ceil(K/a)+1):
        if a*b <= K:
            cnt += K//a//b

print(cnt)
