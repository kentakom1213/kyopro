#         007 - CP Classes（★3）
# ----------------------------------------
# 問題
# https:#atcoder.jp/contests/typical90/tasks/typical90_g

# AC
# ----------------------------------------

from bisect import bisect_left

if __name__ == "__main__":
    N = int(input())
    A = list(map(int, input().split()))
    A.sort()
    Q = int(input())

    for q in range(Q):
        b = int(input())
        i = bisect_left(A, b)
        
        res = 1e10
        if i-1 >= 0: res = min(res, abs(A[i-1] - b))
        if i < N: res = min(res, abs(A[i] - b))
    
        print(res)