from collections import defaultdict

N, Q = map(int, input().split())
graph = defaultdict(set)

for _ in range(Q):
    t, a, b = map(int, input().split())
    a -= 1; b -= 1

    if t == 1:
        graph[a].add(b)
    elif t == 2:
        if b in graph[a]:
            graph[a].remove(b)
    elif t == 3:
        if a in graph[b] and b in graph[a]:
            print("Yes")
        else:
            print("No")
            