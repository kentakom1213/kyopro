from functools import lru_cache
from bisect import bisect_left
import sys
sys.setrecursionlimit(1000000)

N, M = map(int, input().split())
C = []
P = []
SS = []

for i in range(N):
    c, p, *s = map(int, input().split())
    C.append(c)
    P.append(p)
    SS.append(sorted(s))

# i個目のルーレットでj点以上取る確率
prob = [[0] * (M + 1) for _ in range(N)]
for i in range(N):
    for j in range(M + 1):
        left = bisect_left(SS[i], j)
        prob[i][j] = (P[i] - left) / P[i]


@lru_cache
def f(x: int) -> float:
    """得点xを獲得するためにかかる金額の期待値
    """
    if x <= 0:
        return 0
    
    # 最も確率が高い袋を選ぶ
    max_prob = 0
    max_idx = 0
    for i in range(N):
        if prob[i][x] > max_prob:
            max_prob = prob[i][x]
            max_idx = i
    
    # i番目の袋から引いて期待値を出す
    ok = bisect_left(SS[max_idx], x) + 1

    return C[max_idx] * ok
    

# 解の出力
print(f"{f(M):.10f}")
