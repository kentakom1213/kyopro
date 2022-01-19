#        E - Amusement Park
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc216/tasks/abc216_e
# ----------------------------------------

# 大きい方から貪欲に追加していく
# 操作はN回以内に終了する

from collections import defaultdict

N, K = map(int, input().split())
A_vals = defaultdict(int)
for i in input().split():
    A_vals[int(i)] += 1

A = list(A_vals.items())
A.sort()

res = 0
while len(A) >= 2 and K:
    print(A)
    cur, cnt = A.pop()
    nxt, ncnt = A.pop()  # 次に大きい値

    # curからnxtになるまでresを追加
    # 追加する回数
    n = (cur - nxt + 1) * cnt

    if n <= K:
        res += (cur + nxt) * (cur - nxt + 1) // 2
        A.append((nxt, ncnt + cnt))
        K -= n
    else:
        A = [(nxt, ncnt), (cur, cnt)]
        break

# Aは必ず要素を持つ
cur, cnt = A.pop()
n = cnt // K  # 加算できる回数
res += (2 * cur - n) * n // 2
K -= n
cur -= n

print(res)
res += cur * K

print(K)
print(res)
