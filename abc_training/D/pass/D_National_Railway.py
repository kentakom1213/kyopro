#           D - National Railway          
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc210/tasks/abc210_d
# ----------------------------------------

"""
## 方針
- H, W <= 1000
"""

H, W, C = map(int, input().split())
A = [list(map(int, input().split())) for _ in range(H)]

# 愚直に実装 -> ちゃんとTLE
ans = 1e20
for i in range(H):
    for j in range(W):
        for ii in range(H):
            for jj in range(W):
                if i==ii and j==jj:
                    continue
                cost = A[i][j] + A[ii][jj] + C * (abs(i-ii) + abs(j-jj))
                if ans > cost:
                    ans = cost

print(ans)
