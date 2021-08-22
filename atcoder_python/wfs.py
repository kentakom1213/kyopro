# width first search
# https://atcoder.jp/contests/abc007/tasks/abc007_3

from collections import deque

exprint = lambda l: print(*l, sep="\n")

# input
R, C = map(int, input().split())
start = list(map(int, input().split()))
goal = list(map(int, input().split()))

field = [input() for _ in range(R)]

