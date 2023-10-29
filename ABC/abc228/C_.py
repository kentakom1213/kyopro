
N, K = map(int, input().split())
P = [(sum(map(int, input().split())), i) for i in range(N)]

P.sort(reverse=True)

res = [False] * N
# K位との差が何点かを調べる
for p, i in P:
    if P[K-1][0] - p <= 300:
        res[i] = True

for r in res:
    print("Yes" if r else "No")
# print(*P, sep="\n")