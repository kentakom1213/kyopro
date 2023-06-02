# https://atcoder.jp/contests/abc230/tasks/abc230_d

N, D = map(int, input().split())
walls = []
for i in range(N):
    l, r = map(int, input().split())
    walls.append((r, l))

walls.sort()

# 貪欲法
ans = 0
now = -1e10
for r, l in walls:
    if now + D <= l:
        now = r
        ans += 1

print(ans)