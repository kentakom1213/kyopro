
N, M = map(int, input().split())
S = input()
C = [int(v) - 1 for v in input().split()]

# 色で分類
colors = [[] for _ in range(M)]

for s, c in zip(S, C):
    colors[c].append(s)

# print(colors)

ans = []
idxs = [-1 for _ in range(M)]

for i in range(N):
    col = C[i]
    idx = idxs[col]
    ans.append(colors[col][idx])
    idxs[col] += 1

print("".join(ans))
