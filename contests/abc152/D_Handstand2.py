#             D - Handstand2
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc152/tasks/abc152_d
# ----------------------------------------

# 愚直に実装するとO(n^2)となる

def init_array(i, j, val=0): return [[val]*j for _ in range(i)]

N = int(input())

arr = init_array(10, 10, 0)  # arr[i][j] := iで始まり、jで終わるN以下の数の個数

for n in range(1, N+1):
    i, j = int(str(n)[0]), int(str(n)[-1])
    arr[i][j] += 1

cnt = 0
for i in range(1, 10):
    for j in range(1, 10):
        cnt += arr[i][j] * arr[j][i]

print(cnt)
