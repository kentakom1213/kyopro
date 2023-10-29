#             D - おいしいたこ焼きの焼き方            
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc005/tasks/abc005_4
# ----------------------------------------

"""
# 方針
全てのブロックの選びかた、
comb(50, 2) * comb(50, 2) = 1225 ** 2 = 1500625 = 2e+6

あらかじめ前計算しておく

## TLE対策
二次元累積和
"""

from itertools import combinations

N = int(input())
D = [list(map(int, input().split())) for _ in range(N)]

# 二次元累積和
S = [[0]*(N+1) for _ in range(N+1)]
for i in range(N):
    for j in range(N):
        S[i+1][j+1] = S[i][j+1] + S[i+1][j] - S[i][j] + D[i][j]

# クエリの前計算
ans = [0] * 3000

# 全ての区画を計算する
for r1, r2 in combinations(range(N+1), 2):
    for c1, c2 in combinations(range(N+1), 2):
        cell = (r2 - r1) * (c2 - c1)

        # たこ焼きの数を計算
        takoyaki = S[r2][c2] - S[r1][c2] - S[r2][c1] + S[r1][c1]

        if ans[cell] < takoyaki:
            ans[cell] = takoyaki

# より大きい値で置き換え
for i in range(N*N):
    if ans[i+1] < ans[i]:
        ans[i+1] = ans[i]

# クエリの処理
Q = int(input())
for i in range(Q):
    q = int(input())
    print(ans[q])
        
