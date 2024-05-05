# C - Bugged
# -----------
# 問題
# https://atcoder.jp/contests/abc063/tasks/arc075_a

# ゴリ押し感が強い

# AC
# -----------

def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

N = int(input())
S = [int(input()) for _ in range(N)]
MAX = 10000

# dpで解く
dp = init_array(N+1, MAX+100)  # dp[i][j] := i個のテストを受けたとき、j点がありうるかどうか
dp[0][0] = 1

for i, s in enumerate(S):
    for j in range(MAX):
        if dp[i][j] == 0:
            continue

        # sを足す場合
        dp[i+1][j+S[i]] = 1
        
        # sを足さない場合
        dp[i+1][j] = 1

# 最大値を検索
max_score = 0
for i, s in enumerate(dp[N]):
    if s and i % 10:
        max_score = i

print(max_score)

