# https://atcoder.jp/contests/arc034/tasks/arc034_2

def digit_sum(n):
    return sum(map(int, str(n)))

N = int(input())
F_MAX = 9 * 20

ans = set()

for f in range(F_MAX):
    for d in range(1, 18+1):
        x = N - f
        if x > 0 and digit_sum(x) == f:
            ans.add(x)


print(len(ans))
for a in sorted(ans):
    print(a)
