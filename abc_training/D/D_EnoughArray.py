#           D - Enough Array
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc130/tasks/abc130_d
# ----------------------------------------

from collections import deque

N, K = map(int, input().split())
A = list(map(int, input().split()))

# K未満である区間の個数を尺取法で求める
q = deque()
summ = 0
can_ext = [0] * N

for r, a in enumerate(A):
    print(q)
    if summ + a >= K:
        while q and summ + a >= K:
            top, _ = q.popleft()
            summ -= top

            if q and summ < K:
                l = q[0][1]
                can_ext[l] = max(can_ext[l], r)

    q.append((a, r))
    summ += a

    l = q[0][1]
    if summ < K:
        can_ext[l] = r+1

# 全ての区間の数から引く
ans = N * (N-1) // 2
for l, r in enumerate(can_ext):
    if r > l:
        underK = (r-l) * (r-l-1) // 2
        ans -= underK

print(can_ext)
print(ans)
