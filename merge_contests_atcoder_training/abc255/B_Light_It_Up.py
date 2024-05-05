#             B - Light It Up             
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc255/tasks/abc255_b
# ----------------------------------------

# 以下，距離はすべて2乗した値

N, K = map(int, input().split())
A = [int(n)-1 for n in input().split()]
B = [i for i in range(N) if i not in A]  # 明かりを持っていない人
P = [list(map(int, input().split())) for _ in range(N)]

dist_max = 0  # 明かりを持っている人と，持っていない人の距離の最小値の最大値

for i in B:
    dist_min = 1e20
    for j in A:
        d = (P[i][0]-P[j][0])**2 + (P[i][1]-P[j][1])**2
        dist_min = min(dist_min, d)
    dist_max = max(dist_max, dist_min)

print(dist_max ** 0.5)
