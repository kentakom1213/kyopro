N, M = map(int, input().split())
P = list(int(input()) for _ in range(N))

P.append(0) # 選ばなくても良い

# 2本の組合せを全列挙
combi = set()
for i in range(N + 1):
    for j in range(i, N + 1):
        combi.add(P[i] + P[j])
combi = sorted(combi)

ans = 0

# 残りの2本
for ab in combi:
    # Mを超えている場合
    if ab > M:
        break

    ok = -1
    ng = len(combi)
    while ng - ok > 1:
        mid = (ok + ng) // 2
        if ab + combi[mid] <= M:
            ok = mid
        else:
            ng = mid

    if ans < ab + combi[ok] <= M:
        ans = ab + combi[ok]

print(ans)
