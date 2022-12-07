# https://atcoder.jp/contests/abc024/tasks/abc024_a

a, b, c, k = map(int, input().split())
s, t = map(int, input().split())

ans = a*s + b*t - ((s+t)*c if s+t >= k else 0)
print(ans)