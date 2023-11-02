N, M = map(int, input().split())
SS = []
for i in range(M):
    k, *S = list(map(int, input().split()))
    tmp = 0
    for s in S:
        tmp |= 1 << (s - 1)
    SS.append(tmp)

P = list(map(int, input().split()))

# bit全探索
ans = 0

for i in range(1 << N):
    isOK = True
    for idx, s in enumerate(SS):
        sw = bin(i & s).count("1")  # onになっているスイッチの個数
        isOK &= sw % 2 == P[idx]
    ans += isOK

print(ans)