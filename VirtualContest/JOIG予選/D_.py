

def binary_search(condition, min, max):
    while max - min > 1:
        mid = int((max + min) // 2)
        if condition(mid):
            min = mid
        else:
            max = mid
    return min


if __name__ == "__main__":
    N, M, D = map(int, input().split())
    X, V = [0]*N, [0]*N
    for i in range(N):
        X[i], V[i] = map(int, input().split())

    # dp_v[i] := i枚目までの絵画を使って表現できる最大の華やかさ
    # dp_x[i] := 上の値にする時の現在の距離
    dp_v = [0] * N
    dp_x = [0] * N
    dp_v[0] = V[0]
    dp_x[0] = X[0]

    for i in range(1, N):

        # スキップする場合
        dp_v[i] = dp_v[i-1]
        dp_x[i] = dp_x[i-1]

        # 条件を満たす適切な距離を求める
        left = binary_search(lambda x: X[i] - dp_x[x] >= D, -1, i)
        print(i, left)

        # i枚目の絵を貼る場合
        if left != -1 and dp_v[left] + V[i] > dp_v[i-1]:
            dp_v[i] = dp_v[left] + V[i]
            dp_x[i] = X[i]

print(dp_x)
print(dp_v)

