#             D - Handstand
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc124/tasks/abc124_d
# ----------------------------------------

from collections import deque

N, K = map(int, input().split())
S = input()

# ランレングス圧縮
comp = []
now = S[0]
cnt = 0
for c in S:
    if c == now:
        cnt += 1
    else:
        comp.append((int(now), cnt))
        now = c
        cnt = 1
comp.append((int(now), cnt))

# print(comp)

# 最も総和が大きい区間を尺取り法で取得
q = deque()
now_len = 0
ans = 0

for state, n in comp:

    if state:
        q.append((state, n))
        now_len += n
    else:
        while K == 0:
            pop_state, pop_n = q.popleft()
            now_len -= pop_n
            if pop_state == 0:
                K += 1
        # 追加
        q.append((state, n))
        now_len += n
        K -= 1
    

    ans = max(ans, now_len)
    # print(now_len, q)

print(ans)
