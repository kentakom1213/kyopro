#              C - Not Equal
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc209/tasks/abc209_c

# 参考
# https://atcoder.jp/contests/abc209/editorial/2228

# 全然わからなかった

# AC (解説)
# ----------------------------------------

N = int(input())
C = list(map(int, input().split()))

MOD = 1000000007

C.sort()
ans = 1

for i in range(N):
    ans = ans * max(0, C[i] - i) % MOD

print(ans)