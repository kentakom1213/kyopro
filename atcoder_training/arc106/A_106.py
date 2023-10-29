#                A - 106
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc106/tasks/arc106_a
# ----------------------------------------

# log_3_10^18 = 37.72...
# log_5_10^18 = 25.75...

N = int(input())

for a in range(1, 40):
    for b in range(1, 30):
        if pow(3, a) + pow(5, b) == N:
            print(a, b)
            exit()

print(-1)
