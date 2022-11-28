# https://atcoder.jp/contests/abc143/tasks/abc143_d

from bisect import bisect_left

N = int(input())
L = sorted(map(int, input().split()))

ans = 0
for i in range(N):
    for j in range(i+1, N):
        a, b = L[i], L[j]
        rem = L[j+1:]  # 使える棒

        mini = abs(a - b)
        maxi = a + b

        top = bisect_left(rem, maxi)
        bottom = bisect_left(rem, mini)

        ans += top - bottom

print(ans)        
