from collections import Counter

N, K = map(int, input().split())
S = [input() for _ in range(N)]

# bit全探索
ans = 0
for i in range(1 << N):
    ssum = ""
    for j in range(N):
        if (i >> j) & 1:
            ssum += S[j]
    cnt = Counter(ssum)

    tmp = 0
    for v in cnt.values():
        if v == K:
            tmp += 1

    ans = max(ans, tmp)

print(ans)
