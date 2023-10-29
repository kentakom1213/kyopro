N = int(input())
A = [(0, int(v)) for v in input().split()]  # (遅延操作の回数, 現在の値)
Q = int(input())
queries = [tuple(map(int, input().split())) for _ in range(Q)]

# 遅延させる値
delay = 0
cnt = 0

# クエリ処理
for qs in queries:
    if qs[0] == 1:
        delay = qs[1]
        cnt += 1
    elif qs[0] == 2:
        _, i, x = qs
        i -= 1
        d, val = A[i]
        if d < cnt:  # 反映されていない場合
            A[i] = (cnt, delay + x)
        else:        # 反映されている場合
            A[i] = (cnt, val + x)
    elif qs[0] == 3:
        i = qs[1] - 1
        d, val = A[i]
        if d < cnt:  # 遅延があったら反映させる
            A[i] = (cnt, delay)
            print(delay)
        else:
            print(val)
