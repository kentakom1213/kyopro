#          C - Forbidden List
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc170/tasks/abc170_c

# 集中力を欠いている
# ----------------------------------------

from bisect import bisect_left

if __name__ == "__main__":
    X, N = map(int, input().split())

    if N == 0:
        input()
        print(X)
    else:
        p = list(map(int, input().split()))
        p.sort()

        mini = 1e10
        val = 0
        for x in range(0, 201):
            i = bisect_left(p, x)
            if abs(X - x) < mini and (i >= N or p[i] != x):
                mini = abs(X - x)
                val = x
        
        print(val)