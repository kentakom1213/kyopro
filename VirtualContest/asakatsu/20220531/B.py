# https://atcoder.jp/contests/abc222/tasks/abc222_c

N, M = map(int, input().split())
A = [input() for _ in range(2*N)]

res = [[i,0] for i in range(2*N)]

WIN_LEFT = [("G", "C"), ("P", "G"), ("C", "P")]

for i in range(M):
    for j in range(N):

        x, y = 2*j, 2*j+1
        x_hand, y_hand = A[res[x][0]][i], A[res[y][0]][i]

        if (x_hand, y_hand) in WIN_LEFT:
            res[x][1] += 1
        elif (y_hand, x_hand) in WIN_LEFT:
            res[y][1] += 1
    
    res.sort(key=lambda t: (-t[1], t[0]))

for i, w in res:
    print(i+1)
