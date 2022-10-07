# https://atcoder.jp/contests/abc187/tasks/abc187_d

N = int(input())
diff = [0] * N
vote = 0
for i in range(N):
    a, b = map(int, input().split())
    vote -= a
    diff[i] = 2*a + b

diff.sort()

ans = 0
while vote <= 0:
    x = diff.pop()
    vote += x
    ans += 1

print(ans)