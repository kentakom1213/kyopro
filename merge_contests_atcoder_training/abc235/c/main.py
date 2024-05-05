
N, Q = map(int, input().split())
A = list(map(int, input().split()))
queries = [tuple(map(int, input().split())) for _ in range(Q)]

dic = {}
for i, a in enumerate(A):
    if a in dic:
        dic[a].append(i)
    else:
        dic[a] = [i]

for x, k in queries:
    if x in dic:
        if len(dic[x]) < k:
            print(-1)
        else:
            print(dic[x][k-1] + 1)
    else:
        print(-1)
