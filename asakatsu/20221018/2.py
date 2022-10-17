# https://atcoder.jp/contests/abc026/tasks/abc026_b

N = int(input())
rs = sorted([int(input()) for _ in range(N)], reverse=True)
sq = sum( (-1)**i * v**2 for i,v in enumerate(rs) )
print(sq * 3.1415926535897932)
