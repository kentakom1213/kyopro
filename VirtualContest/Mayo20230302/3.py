# https://atcoder.jp/contests/abc136/tasks/abc136_d

# よくわかんないからダブリング

from collections import Counter

LOG = 333  # 2^333 > 10^100

S = input()
N = len(S)

# ダブリング配列
# dp[i][j] := i文字目の人が2^j回移動したときにいる場所
dp = [[0] * N for _ in range(LOG)]

for j, d in enumerate(S):
    if d == "R":
        dp[0][j] = j + 1
    else:
        dp[0][j] = j - 1

for i in range(1, LOG):
    for j in range(N):
        dp[i][j] = dp[i-1][dp[i-1][j]]

# カウント
cnt = Counter(dp[-1])

for i in range(N):
    print(cnt[i], end=" ")
print()
