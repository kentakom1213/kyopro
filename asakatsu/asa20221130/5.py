# https://atcoder.jp/contests/arc025/tasks/arc025_2

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


H, W = map(int, input().split())
C = [list(map(int, input().split())) for _ in range(H)]

for i in range(H):
    for j in range(W):
        if (i+j) % 2 == 1:
            C[i][j] *= -1

# 二次元累積和
get = Acc2D(C)

# 全ての矩形領域を探索
ans = 0
for i in range(H):
    for j in range(i+1, H+1):
        for k in range(W):
            for l in range(k+1, W+1):
                tmp = (j - i) * (l - k)
                p = get(i, j, k, l)

                if p == 0:
                    ans = max(ans, tmp)
print(ans)
