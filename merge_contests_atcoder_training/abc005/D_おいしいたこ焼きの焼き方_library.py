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

→ ライブラリを使用
"""

from itertools import combinations

def Acc2D(arr):
    """
    二次元累積和
    """
    H, W = len(arr), len(arr[0])  # 配列の横、縦
    S = [[0]*(W+1) for _ in range(H+1)]  # 累積和配列
    for i in range(H):
        for j in range(W):
            S[i+1][j+1] = S[i][j+1] + S[i+1][j] - S[i][j] + arr[i][j]
    
    def get(row_l, row_r, col_l, col_r):
        """
        `arr[row_l:row_r, col_l:col_r]`の要素の和を返す
        """
        return S[row_r][col_r] - S[row_r][col_l] - S[row_l][col_r] + S[row_l][col_l]
    
    return get

N = int(input())
D = [list(map(int, input().split())) for _ in range(N)]

# 二次元累積和
get = Acc2D(D)

# クエリの前計算
ans = [0] * 3000

# 全ての区画を計算する
for r1, r2 in combinations(range(N+1), 2):
    for c1, c2 in combinations(range(N+1), 2):
        cell = (r2 - r1) * (c2 - c1)

        # たこ焼きの数を計算
        takoyaki = get(r1, r2, c1, c2)

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
        
