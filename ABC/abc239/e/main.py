
N, Q = map(int, input().split())
X = list(map(int, input().split()))

G = [[] for _ in range(N)]
for i in range(N-1):
    a, b = map(int, input().split())
    a-=1; b-=1
    G[a].append(b)
    G[b].append(a)

queries = [tuple(map(int, input().split())) for _ in range(Q)]
