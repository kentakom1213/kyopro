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

points = [0] * (2*N)
for i in range(N):
    a, b = map(int, input().split())
    points[2*i] = b
    points[2*i+1] = a - b  # 残りの方の点数を取る
    
points.sort(reverse=True)

# 貪欲に取る
print(sum(points[:K]))
