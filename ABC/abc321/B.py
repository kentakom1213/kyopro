
N, X = map(int, input().split())
A = list(map(int, input().split()))

# 解を全探索する
for p in range(0, 101):
    # スコアを計算
    S = A + [p]
    score = sum(sorted(S)[1:-1])

    if score >= X:
        print(p)
        exit()

print(-1)
