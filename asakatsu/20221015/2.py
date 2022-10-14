# https://atcoder.jp/contests/abc036/tasks/abc036_b

N = int(input())
x = [input() for _ in range(N)]

for r in zip(*x):
    print("".join(r[::-1]))