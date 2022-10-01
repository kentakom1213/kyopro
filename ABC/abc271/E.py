
N, M, K = map(int, input().split())

G = [[] for _ in range(N)]
for i in range(M):
    a, b, c = map(int, input().split())
    G[a].append((b, c))
    G[b].append((a, c))

E = list(map(int, input().split()))


