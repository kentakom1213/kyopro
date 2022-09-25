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
