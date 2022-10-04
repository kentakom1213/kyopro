# https://atcoder.jp/contests/agc024/tasks/agc024_a

a, b, c, k = map(int, input().split())

ans = a - b if k == 0 else (b - a) * k
print(ans)

if abs(ans) > 1000_000_000_000_000_000:
    print("Unfair")
else:
    print(ans)