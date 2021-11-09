#     048 - I will not drop out（★3）
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/typical90/tasks/typical90_av

# 参考
# https://pbs.twimg.com/media/E2L_lV9UcAMmR5a?format=jpg&name=large
# ----------------------------------------

# むずい
# 貪欲にとる

N, K = map(int, input().split())

points = [tuple(map(int, input().split())) for _ in range(N)]

points.sort()
res = 0
while K:
