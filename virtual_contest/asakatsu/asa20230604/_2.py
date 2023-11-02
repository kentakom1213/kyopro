N, X, Y, Z = map(int, input().split())
A = list(map(int, input().split()))
B = list(map(int, input().split()))

AB = list(zip(range(1, N + 1), A, B))
AB.reverse()

passed = set()

# 数学の点数で点数でとる
AB.sort(key=lambda x: (x[1], -x[0]))

for _ in range(X):
    if AB:
        idx, *_ = AB.pop()

        passed.add(idx)

# 英語の点数でとる
AB.sort(key=lambda x: (x[2], -x[0]))

for _ in range(Y):
    if AB:
        idx, *_ = AB.pop()
        passed.add(idx)

# 合計点でとる
AB.sort(key=lambda x: (x[1] + x[2], -x[0]))

for _ in range(Z):
    if AB:
        idx, *_ = AB.pop()
        passed.add(idx)


# 出力
print(*sorted(passed))
