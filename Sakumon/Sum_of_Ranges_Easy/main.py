
import sys
input = sys.stdin.readline

# input
N, Q = map(int, input().split())
A = list(map(int, input().split()))

# クエリの処理（愚直に和をとる）
for _ in range(Q):
    l, r = map(int, input().split())
    l -= 1

    ans = 0
    for i in range(l, r):
        ans += A[i]

    print(ans)
