from math import sqrt

def dist(us, vs):
    x1, y1 = us
    x2, y2 = vs
    return sqrt((x1-x2)**2 + (y1-y2)**2)

N, M = map(int, input().split())
ALL = [tuple(map(int, input().split())) for _ in range(N+M)]

# 定数
L = N + M
INF = 1e20

# dp[i][j] := 頂点の集合Sを周ったとき、最短で到達できる時間
dp = [[INF]*L for _ in range(1 << L)]
for i in range(L):
    dp[0][i] = 0

# bit全探索
for S in range(1 << L):
    # u -> v に移動
    for v in range(N):
        for u in range(N):

            # uを通っていない場合は無視
            if S != 0 and (S & (1 << u) == 0):
                continue
        
            # 同じ頂点の場合は無視
            if u == v:
                continue
            
            # 現在の速度を計算
            speed = 1
            for box in range(N, L):
                if (S >> box) & 1:
                    speed *= 2
            
            old_time = dp[S | (1 << v)][v]
            new_time = dp[S][u] + dist(ALL[u], ALL[v]) / speed
            if old_time > new_time:
                dp[S | (1 << v)][v] = new_time

# 全ての都市を周り切った中で、最短のタイム
ans = INF

ALL_CITY = (1 << N) - 1  # 全ての都市を周った集合

for i in range(1<<M):
    for j in range(M):
        if (i >> j) & 1:
            # 最終到達点での時間
            last_time = dp[ALL_CITY | i][j]

            # 現在の速度を計算
            speed = 1
            for box in range(N, L):
                if (S >> box) & 1:
                    speed *= 2

            # 帰ってくる時間
            return_time = last_time + dist(ALL[j], (0, 0)) / speed + dist(ALL[j], (0, 0))

            if ans > return_time:
                ans = return_time

print(ans)