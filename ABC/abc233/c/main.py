
from bisect import bisect_left

N, X = map(int, input().split())

balls = []
counts = []

for i in range(N):
    L, *A = map(int, input().split())
    dic = {a:A.count(a) for a in A}   # <-- ここでTLE
    dic = sorted(dic.items(), key=lambda x: x[0])
    nums = [num for num, _ in dic]
    cnts = [cnt for _, cnt in dic]

    L = len(dic)
    # 累積和
    S = [0] * L
    S[0] = cnts[0]
    for i in range(1, L):
        S[i] = S[i-1] + cnts[i]

    balls.append(nums)
    counts.append(S)

# print(*balls, sep="\n")

# Xの素因数分解はできない
# 深さ優先探索？
# いい感じに枝刈り
# にぶたんで

cnt = 0
def dfs(i, prod=1, pattern=1):
    global cnt
    if i == N:
        if prod == X:
            cnt += pattern
        return

    rest = X / prod
    print(prod,rest)
    max_index = bisect_left(balls[i], rest)

    


dfs(0)
print(cnt)

