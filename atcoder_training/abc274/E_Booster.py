#               E - Booster               
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc274/tasks/abc274_e
# ----------------------------------------

# 解説
# https://atcoder.jp/contests/abc274/editorial/5020

INF = 1e20

def get_speed(S):
    speed = 1
    for i in range(N, M):
        if (S >> i) & 1:
            speed *= 2
    return speed

def dist(us, vs):
    x1, y1 = us
    x2, y2 = vs
    return ((x1-x2)**2 + (y1-y2)**2) ** 0.5

N, M = map(int, input().split())
M += N
A = [tuple(map(int, input().split())) for _ in range(M)]

# DP[i][S]=現在地が街 i で、訪問済みの街の集合が S であるときの所要時間の最小値
dp = [[INF]*(1<<M) for _ in range(M)]
for i in range(M):
    dp[i][1<<i] = dist(A[i], (0, 0))

for S in range(1 << M):
    speed = get_speed(S)
    for v in range(M):
        for u in range(M):
            # uを訪れていない場合を無視
            if S != 0 and not(S & (1 << u)):
                continue
            # u == v の場合を無視
            if u == v:
                continue

            # u -> v を更新
            old_time = dp[v][S | (1 << v)]
            new_time = dp[u][S] + dist(A[u], A[v]) / speed

            if new_time < old_time:
                dp[v][S | (1 << v)] = new_time

# print(*dp, sep="\n")

ans = INF
for i in range(M):
    for S in range((1<<N)-1, 1<<M, 1<<N):
        ans = min(
            ans,
            dp[i][S] + dist(A[i], (0, 0)) / get_speed(S)  # 帰り道を足す
        )

print(ans)
