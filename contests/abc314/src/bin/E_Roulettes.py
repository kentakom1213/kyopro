#              E - Roulettes              
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc314/tasks/abc314_e
# ----------------------------------------

import sys
from functools import lru_cache

sys.setrecursionlimit(1_000_000_000)

N, M = map(int, input().split())
C = []
P = []
S = []
for _ in range(N):
    c, p, *s = map(int, input().split())
    C.append(c)
    P.append(p)
    S.append(s)

# 0を除いて金額を更新
for i in range(N):
    C[i] *= P[i]
    S[i] = [s for s in S[i] if s != 0]  # 0を削除
    P[i] = len(S[i])  # 要素数を更新
    C[i] /= P[i]

@lru_cache
def e(i: int) -> float:
    if i >= M:
        return 0
    
    # すべてのルーレットに関して試す
    res = 1e20
    for j in range(N):
        tmp = 0
        for k in range(P[j]):
            tmp += e(i + S[j][k])
        tmp /= P[j]
        tmp += C[j]

        if res > tmp:
            res = tmp

    return res

print(f"{e(0):.10f}")
