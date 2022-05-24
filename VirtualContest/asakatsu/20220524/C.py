# https://atcoder.jp/contests/arc014/tasks/arc014_2

from collections import defaultdict

N = int(input())
words = [input() for _ in range(N)]

dd = defaultdict(int)
for i, w in enumerate(words):
    if w in dd:
        print("WIN" if i&1 else "LOSE")
        exit()
    if i and words[i-1][-1] != w[0]:
        print("WIN" if i&1 else "LOSE")
        exit()
    dd[w] += 1

print("DRAW")
