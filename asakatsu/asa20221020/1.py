# https://atcoder.jp/contests/abc210/tasks/abc210_b

N = int(input())
A = list(input())

lose = 0
for i, a in enumerate(A):
    if a ==  "1":
        lose = i & 1
        break

print("Takahashi" if lose == 0 else "Aoki")