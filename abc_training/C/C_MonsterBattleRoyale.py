#       C - Monsters Battle Royale
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc118/tasks/abc118_c

# 焦らない

# AC
# ----------------------------------------

# 計算量は　O(n)

import heapq

if __name__ == "__main__":
    N = int(input())
    A = list(map(int, input().split()))
    heapq.heapify(A)

    while len(A) > 1:
        mini = heapq.heappop(A)

        while A and A[0] > mini:
            val = heapq.heappop(A) % mini
            if val != 0:
                heapq.heappush(A, val)

        if A and A[0] != mini:
            heapq.heappush(A, mini)
    
    print(mini)
