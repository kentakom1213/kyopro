# https://atcoder.jp/contests/abc219/tasks/abc219_c

from string import ascii_lowercase as al

X = input()
N = int(input())
ss = [input() for _ in range(N)]

xo = {c:i for i, c in enumerate(X)}  # X's order

ss.sort(key=lambda s: "".join(al[xo[c]] for c in s))
print(*ss, sep="\n")
