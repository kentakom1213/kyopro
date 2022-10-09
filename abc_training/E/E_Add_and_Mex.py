#             E - Add and Mex             
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc272/tasks/abc272_e
# ----------------------------------------

N, M = map(int, input().split())
A = list(map(int, input().split()))

# "大事な値"を調べる
vals = [[] for _ in range(M+1)]
for i in range(N):
    if A[i] >= N:
        continue

    # `A[i] + i*(j+1)` が0以上N未満になる範囲を探索
    l = A[i] >= 0 or (-A[i] + i) // (i + 1)
    r = min(M+1, (N - A[i] + i) // (i + 1))
    for j in range(l, r):
        vals[j].append(A[i] + (i+1) * j)

# mの値それぞれについて、含まれない最小の非負整数を求める
for i, v in enumerate(vals):
    if i == 0: continue
    exists = [False] * (len(v) + 1)
    for x in v:
        if x < len(v)+1:
            exists[x] = True
    res = 0
    while exists[res]:
        res += 1
    print(res)