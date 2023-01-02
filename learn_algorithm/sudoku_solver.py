
def solve(arr, cell):
    """単純なdfsで解く"""

    # 求まったとき
    if cell == 81:
        print(*arr, sep="\n")
        exit()

    i, j = cell//9, cell%9
    if arr[i][j] == 0:
        # 順に代入する
        for n in range(1, 10):

            # --- 行 ---
            if n in arr[i]:
                continue

            # --- 列 ---
            is_in_col = False
            for r in range(9):
                is_in_col |= arr[r][j] == n
            if is_in_col:
                continue

            # --- ブロック ---
            is_in_block = False
            for r in range(i//3*3, i//3*3+3):
                for c in range(j//3*3, j//3*3+3):
                    is_in_block |= arr[r][c] == n
            if is_in_block:
                continue

            # 条件をみたしていたとき
            arr[i][j] = n
            solve(arr, cell+1)
            arr[i][j] = 0
    else:
        solve(arr, cell+1)



def main():
    arr = [list(map(int, input().split())) for _ in range(9)]

    solve(arr, 0)


if __name__ == "__main__":
    main()
