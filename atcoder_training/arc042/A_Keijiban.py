#               A - 掲示板
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc042/tasks/arc042_a
# ----------------------------------------

n, m = map(int, input().split())
A = [int(input()) for _ in range(m)]

remaining = set(range(1, n+1))
ans = []
for a in reversed(A):
    if a in remaining:
        ans.append(a)
        remaining.remove(a)

for rem in sorted(remaining):
    ans.append(rem)

print(*ans, sep="\n")
