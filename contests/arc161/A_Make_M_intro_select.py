#                A - Make M
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc161/tasks/arc161_a
# ----------------------------------------

def partition(arr, pvt):
    p_idx = None
    for i, val in enumerate(arr):
        if val == pvt:
            p_idx = i
    if p_idx is None:
        return
    
    # pivotを末尾に
    arr[p_idx], arr[-1] = arr[-1], arr[p_idx]
    p_idx = -1

    # 順に交換
    for i, val in enumerate(arr[:-1]):
        if val <= pvt:
            p_idx += 1
            arr[p_idx], arr[i] = arr[i], arr[p_idx]
    
    # pivotを戻す
    arr[p_idx + 1], arr[-1] = arr[-1], arr[p_idx + 1]

    return p_idx + 1


def select(arr, k):
    """
    配列arrのk番目の要素を取得
    """

    if len(arr) == 1:
        return arr[0]

    # ピボット
    pvt = pivot(arr)

    # ピボットのある位置
    pvt_idx = partition(arr, pvt)

    # ピボットの位置で場合分け
    if pvt_idx < k:
        return select(arr[pvt_idx + 1:], k - pvt_idx - 1)
    elif pvt_idx > k:
        return select(arr[:pvt_idx], k)
    else:
        return arr[k]


def pivot(arr):
    """
    arrを5個ずつに分割した配列の中央値の中央値を求める
    """

    if len(arr) == 1:
        return arr[0]

    size = (len(arr) + 4) // 5
    medians = [0] * size

    for i in range(size):
        sub = arr[5 * i: 5 * (i + 1)]

        # 5この要素の中央値
        med = sorted(sub)[(len(sub) - 1) // 2]

        medians[i] = med
    
    # 中央値の中央値を検索
    return select(medians, (size - 1) // 2)


def main():
    N = int(input())
    A = list(map(int, input().split()))

    if N == 1:
        print("Yes")
        return

    x = select(A, N // 2 + 1)
    cnt = A.count(x)

    if cnt > N / 2:
        print("No")
    else:
        print("Yes")


main()
