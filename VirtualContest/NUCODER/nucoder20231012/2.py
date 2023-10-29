# https://atcoder.jp/contests/abc209/tasks/abc209_c

MOD = 1000000007

N = int(input())
C = list(map(int, input().split()))

# ソート
C.sort()

cur = C[0]
ans = 1

for i in range(N):
    cur += (C[i] - cur) - i
    ans = ans * cur % MOD

print(ans)
