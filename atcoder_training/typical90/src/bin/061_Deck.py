#            061 - Deck（★2）
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/typical90/tasks/typical90_bi

# AC
# ----------------------------------------

from collections import deque

if __name__ == "__main__":
    Q = int(input())
    queries = [tuple(map(int, input().split())) for _ in range(Q)]

    deq = deque()

    for q in queries:
        t, x = q

        if t == 1:
            deq.appendleft(x)
        
        elif t == 2:
            deq.append(x)

        else:
            print(deq[x-1])