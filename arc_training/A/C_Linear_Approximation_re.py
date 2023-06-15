#         C - Linear Approximation
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc102/tasks/arc100_a
# ----------------------------------------

def partition(arr, k, l=0, r=None):
    """
    配列arrのl番目からr-1番目までの要素をk未満とk以上に分割
    """
    if r is None:
        r = len(arr)

    r -= 1

    while l < r:
        if arr[l] < k:
            l += 1
        if arr[r] >= k:
            r -= 1
        if arr[l] >= k and arr[r] < k:
            arr[r], arr[l] = arr[l], arr[r]

    return l


def select(arr, k, l=0, r=None):
    """
    配列arrのk番目の要素を取得
    """

    if r is None:
        r = len(arr)

    if l >= r:
        return
    elif l + 1 == r:
        return arr[l]

    while True:
        # ピボットを選択
        pvt = pivot(arr, l, r)

        # ピボットが何番目にあるか
        pvt_idx = partition(arr, pvt, l, r)

        if pvt_idx < k:
            l = pvt_idx + 1
        elif pvt_idx == k:
            break
        else:
            r = pvt_idx - 1

    return arr[k]


def pivot(arr, l=0, r=None):
    """
    arrを5個ずつに分割した配列の中央値の中央値を求める
    """

    if r is None:
        r = len(arr)

    if l >= r:
        return
    elif l + 1 == r:
        return arr[l]

    size = (r - l + 4) // 5
    medians = [0] * size

    for i in range(size):
        start = 5 * i + l
        end = min(start + 5, r)

        # 5この要素の中央値
        med = sorted(arr[start:end])[(end - start) // 2]

        medians[i] = med

    # 中央値の中央値を検索
    return select(medians, size // 2)


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
