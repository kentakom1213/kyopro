# https://atcoder.jp/contests/arc130/tasks/arc130_b

"""
クエリを先よみして反転？
"""

from collections import defaultdict

H, W, C, Q = map(int, input().split())
queries = [0]*Q
for i in range(Q-1, -1, -1):
    t,n,c = map(int, input().split())
    queries[i] = (t,n,c)

# すでに使われた行、列を保存
row = set()
col = set()
d = defaultdict(int)

for t,n,c in queries:
    if t == 1:
        if n not in col:
            d[c] += W - len(row)
        col.add(n)
    else:
        if n not in row:
            d[c] += H - len(col)
        row.add(n)
    # print(f"{row=}, {col=}, {d=}")

for i in range(1, C+1):
    print(d[i], end=" ")
print()
