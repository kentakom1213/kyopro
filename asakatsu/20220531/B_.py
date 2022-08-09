
# シミュレーション

from operator import itemgetter

WIN_LEFT = [("G", "C"), ("P", "G"), ("C", "P")]

N, M = map(int, input().split())
A = [input() for _ in range(2*N)]

win = [[i, 0] for i in range(2*N)]

for i in range(M):
    for j in range(N):
        a, b = 2*j, 2*j+1
        hand_a, hand_b = A[win[a][0]][i], A[win[b][0]][i]

        if (hand_a, hand_b) in WIN_LEFT:
            win[a][1] += 1
        elif (hand_b, hand_a) in WIN_LEFT:
            win[b][1] += 1
    
    win.sort()
    win.sort(key=itemgetter(1), reverse=True)


win.sort()

win.sort(key=itemgetter(1), reverse=True)

for i, _ in win:
    print(i+1)