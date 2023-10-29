
N, X = map(int, input().split())
A = []
for i in range(N):
    _, *a = map(int, input().split())
    A.append(a)

res = 0
def dfs(i, prod):
    global res
    if i == N:
        if prod == X:
            res += 1
        return
    
    for a in A[i]:
        if a*prod <= X and X % a == 0:
            dfs(i+1, a*prod)

dfs(0, 1)
print(res)