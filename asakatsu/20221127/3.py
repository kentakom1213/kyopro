# https://atcoder.jp/contests/arc011/tasks/arc011_1

m, n, N = map(int, input().split())

ans = N
while N >= m:
    make = (N // m) * n
    ans += make
    N -= (N // m) * m
    N += make

print(ans)
