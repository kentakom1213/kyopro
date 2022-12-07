# https://atcoder.jp/contests/abc100/tasks/abc100_c

N = int(input())
A = list(map(int, input().split()))

# 2で割れる回数の合計を求める。 O(NlogN)

ans = 0
for a in A:
    while a%2 == 0:
        a >>= 1
        ans += 1

print(ans)
