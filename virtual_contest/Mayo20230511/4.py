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

# main
H, W, N, h, w = map(int, input().split())
A = [[int(v) - 1 for v in input().split()] for _ in range(H)]

# 色を分解
mat = [[[0] * W for _ in range(H)] for _ in range(N)]
for i in range(H):
    for j in range(W):
        color = A[i][j]
        mat[color][i][j] = 1

# 色ごとに2次元累積和
acc_colors = [Acc2D(mat[i]) for i in range(N)]

# 全ての色の要素数を保存
col_cnt = [acc_colors[i](0, H, 0, W) for i in range(N)]

# クエリ処理
for i in range(H - h + 1):
    for j in range(W - w + 1):
        kinds = 0
        for c in range(N):
            cnt = acc_colors[c](i, i+h, j, j+w)
            kinds += bool(col_cnt[c] - cnt)
        print(kinds, end=" ")
    print()
