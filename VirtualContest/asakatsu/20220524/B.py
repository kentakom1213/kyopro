# https://atcoder.jp/contests/abc083/tasks/abc083_b

N, A, B = map(int, input().split())

ans = 0
for i in range(1, N+1):
    x = sum(map(int, str(i)))
    if A <= x <= B:
        ans += i

print(ans)