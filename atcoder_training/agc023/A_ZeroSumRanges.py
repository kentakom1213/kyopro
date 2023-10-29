#          A - Zero-Sum Ranges
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/agc023/tasks/agc023_a

# AC (解説)
# ----------------------------------------

# 累積和をとって普通に調べるとO(n^2)
# 累積和の配列から、同じものをえらぶ場合の数と言える
# ランレングス圧縮でもできるけど、差分取って行った方が楽だよね

N = int(input())
A = list(map(int, input().split()))

# 累積和
S = [0] * (N+1)
for i in range(N):
    S[i+1] = S[i] + A[i]

S.sort()

cnt = dup = 0
pre = None
for n in S:
    if pre == n:
        dup += 1
    else:
        dup = 0
    cnt += dup
    pre = n

print(cnt)