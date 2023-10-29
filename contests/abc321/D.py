from bisect import bisect_left

N, M, P = map(int, input().split())
A = list(map(int, input().split()))

# ソートして受け取り
B = sorted(map(int, input().split()))
# Bの累積和
S = [0] * (M + 1)
for i in range(M):
    S[i + 1] = S[i] + B[i]

ans = 0

for a in A:
    # a + b <= Pとなるbの範囲を2分探索
    i = bisect_left(B, P - a)
    # print(i, B[:i], B[i:])

    # Pより小さいやつ
    ans += a * i + S[i]

    # Pより大きいやつ
    ans += P * (M - i)

    # print(a * i + S[i], P * (M - i))

print(ans)
