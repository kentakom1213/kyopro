# https://atcoder.jp/contests/agc025/tasks/agc025_a

def digit_sum(x):
    return sum(map(int, str(x)))

n = int(input())

ans = 1e20
for a in range(1, n):
    b = n - a
    tmp = digit_sum(a) + digit_sum(b)
    if ans > tmp:
        ans = tmp

print(ans)
