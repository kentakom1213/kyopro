#         D - Preparing Boxes
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc134/tasks/abc134_d
# ----------------------------------------

"""comment
- 逆順に貪欲？
- 計算量は調和級数に従う
- 2で割ったあまりだから、偶奇を保持すれば良い
"""

N = int(input())
A = [int(n) for n in input().split()]

ans = [0] * N

for i in range(N, 0, -1):
    s = 0
    for j in range(i-1, N, i):
        s ^= ans[j]
    
    if s != A[i-1]:
        ans[i-1] = 1

# 出力
ansN = 0
for i in range(N):
    if ans[i]:
        ansN = i+1

print(ansN)
if ansN:
    print(*ans[:ansN])
