#             C - Tsundoku
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc172/tasks/abc172_c
# ----------------------------------------

from collections import deque

N, M, K = map(int, input().split())
A = [int(n) for n in input().split()]
B = [int(n) for n in input().split()]

# Aを反転して結合
C = A[::-1] + B

# 尺取法
deq = deque()
ans = now = 0
for i, book in enumerate(C):
    while K < book and now:
        K += deq.popleft()
        now -= 1

    if K >= book:
        deq.append(book)
        K -= book
        now += 1

    # 中心を含んでいれば更新
    if i-now <= N-1 <= i:  # ←ここの判定むずすぎない??
        ans = max(ans, now)

print(ans)
