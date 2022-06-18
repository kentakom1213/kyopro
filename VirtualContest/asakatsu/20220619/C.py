# https://atcoder.jp/contests/arc015/tasks/arc015_2

N = int(input())

ans = [0] * 6
for _ in range(N):
    high, low = map(float, input().split())

    if high >= 35:
        ans[0] += 1
    if 30 <= high < 35:
        ans[1] += 1
    if 25 <= high < 30:
        ans[2] += 1
    if low >= 25:
        ans[3] += 1
    if low < 0 and high >= 0:
        ans[4] += 1
    if high < 0:
        ans[5] += 1

print(*ans)
