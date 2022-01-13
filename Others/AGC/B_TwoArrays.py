# B - Two Arrays
# ---------------
# 問題
# https://atcoder.jp/contests/apc001/tasks/apc001_b
# ---------------

# A, Bの差をとって考える
# diffのうち2項に+2, -1を同時に加える

N = int(input())
A = list(map(int, input().split()))
B = list(map(int, input().split()))

diff = [a-b for a, b in zip(A, B)]

cnt = -sum(diff)

# 処理ができないためNo
if cnt < 0:
    print("No")
    exit()

s, t = 0, 0  # +2, -1に相当するポインタ
for i in range(cnt):
    # 貪欲に処理
    while diff[s] >= 0: s += 1
    while t < N and diff[t] <= 0: t += 1
    if t < N:
        diff[s] += 2
        diff[t] -= 1
    else:
        diff[s] += 1

print("No" if any(diff) else "Yes")

