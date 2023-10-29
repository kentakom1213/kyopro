#              D - Line++
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc160/tasks/abc160_d

# AC
# ----------------------------------------

# O(N^2) でも大丈夫
# なんかもうちょい計算量落とせそうだけど

N, X, Y = map(int, input().split())
X-=1; Y-=1

ans = [0] * N
for i in range(N):
    for j in range(i+1, N):
        shortest = min(
            j - i,  # 通常のルート
            abs(i-X) + abs(j-Y) + 1
        )

        ans[shortest] += 1

for i in range(1, N):
    print(ans[i])
