

N, X = map(int, input().split())
balls = []

for i in range(N):
    _, *A = map(int, input().split())
    A = [a for a in A if X % a == 0]

    dic = {a:A.count(a) for a in A}
    dic = sorted(dic.items(), key=lambda x: x[0])
    balls.append(dic)


cnt = 0
def dfs(i, prod, pattern):
    global cnt
    if i == N:
        if prod == X:
            cnt += pattern
        return

    for ball, num in balls[i]:
        if ball * prod <= X:
            dfs(i+1, ball*prod, pattern*num)


dfs(0, 1, 1)
print(cnt)

