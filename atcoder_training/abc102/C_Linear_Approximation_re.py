#         C - Linear Approximation
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc102/tasks/arc100_a
# ----------------------------------------

def partition(arr, pivot):
    p_cnt = 0
    p_idx = None
    for i, val in enumerate(arr):
        if val == pivot:
            p_idx = i
            p_cnt += 1
    if p_idx is None:
        return
    
    # pivotを末尾に
    arr[p_idx], arr[-1] = arr[-1], arr[p_idx]
    p_idx = 0

    # 順に交換
    for i, val in enumerate(arr[:-1]):
        if val <= pivot:
            arr[p_idx], arr[i] = arr[i], arr[p_idx]
            p_idx += 1

    # pivotを戻す
    arr[p_idx], arr[-1] = arr[-1], arr[p_idx]

    # 左、右を調べる
    left = p_idx + 1 - p_cnt
    right = p_idx
    mid = (len(arr) - 1) // 2
    if mid < left:
        idx = left
    elif mid <= right:
        idx = mid
    else:
        idx = right
    
    # pivotを戻す
    arr[idx], arr[p_idx] = arr[p_idx], arr[idx]

    return idx


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
    A = [int(v) for v in input().split()]

    # iを引く
    A = [a - i for i, a in enumerate(A)]

    # 中央値をとる
    med = select(A, (N - 1) // 2)

    # 中央値からの偏差の和をとる
    ans = 0
    for i in range(N):
        ans += abs(A[i] - med)

    print(ans)


main()
