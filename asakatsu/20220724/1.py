# https://atcoder.jp/contests/abc178/tasks/abc178_b

a,b,c,d = map(int, input().split())
print(max(a*c, b*d, a*d, b*c))
