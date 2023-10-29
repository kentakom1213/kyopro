#        D - Destroyer Takahashi
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc230/tasks/abc230_d

# AC
# ----------------------------------------

# 区間スケジューリング問題

N, D = map(int, input().split())
walls = [tuple(map(int, input().split())) for _ in range(N)]

# 右端でソート
walls.sort(key=lambda x: x[1])

# 貪欲にとっていく
cnt = 0
x = -1e10
for l, r in walls:
    p = x + D - 1
    if l > p:
        x = r
        cnt += 1

print(cnt)
