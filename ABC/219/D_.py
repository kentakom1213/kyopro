
def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

N = int(input())
X, Y = map(int, input().split())
lunchboxes = [tuple(map(int, input().split())) for _ in range(N)]

INF = 1e9
DP = init_array(X, Y, INF)
DP[0][0] = 0




# DP[i][j] = たこ焼きi個、たい焼きj個を手に入れるのに必要な弁当箱の最小値
# DP[i+1][j] = max()

# -> ダメっぽい

# ナップサック問題に帰着できないか