# B - 島と橋
# -----------
# 問題
# https://atcoder.jp/contests/abc027/tasks/abc027_b
# -----------

# 島一つあたりの人口: ppi = sum(A) / N
# A[i]について、A[i]の左側が ppi*i かつ A[i]==ppi のとき適正
# それ以外のとき橋をかける

from itertools import accumulate

N = int(input())
A = list(map(int, input().split()))

S = [0]+list(accumulate(A))

if S[-1] % N != 0:
    print(-1)
    exit()

ppi = S[-1]/N

# 貪欲にとく
cnt = 0
is_connected = False
for i, a in enumerate(A):
    isOK = A[i]==ppi and S[i+1]==ppi*(i+1) \
           or is_connected and S[i+1]==ppi*(i+1)

    if isOK:
        is_connected = False
    else:
        cnt += 1
        is_connected = True

print(-1 if is_connected else cnt)

