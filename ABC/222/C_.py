
# AC
# シミュレーション

from operator import itemgetter

def janken(a, b):
    # あいこ
    if a == b:
        return 0
    # aの負け
    elif (a, b) in [("G", "P"), ("P", "C"), ("C", "G")]:
        return 2
    # aの負け
    else:
        return 1

N, M = map(int, input().split())
A = [input() for _ in range(2*N)]

win = [[i, 0] for i in range(2*N)]

for i in range(M):
    for j in range(N):
        a, b = 2*j, 2*j+1
        hand_a, hand_b = A[win[a][0]][i], A[win[b][0]][i]

        result = janken(hand_a, hand_b)
        # print(f"ラウンド{i+1}: {win[a][0]+1}vs{win[b][0]+1} = {hand_a}:{hand_b} => {result}")

        if result == 1:
            win[a][1] += 1
        elif result == 2:
            win[b][1] += 1
    
    win.sort()
    win.sort(key=itemgetter(1), reverse=True)


win.sort()

win.sort(key=itemgetter(1), reverse=True)

for i, _ in win:
    print(i+1)