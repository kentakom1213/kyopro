#          C - Colorful Candies
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc210/tasks/abc210_c

# 参考
# https://blog.hamayanhamayan.com/entry/2021/07/17/233052

# AC (解説)
# ----------------------------------------


N, K = map(int, input().split())
C = list(map(int, input().split()))

# セットによる実装
# O(N*K)
# var = [len(set(C[i:i+K])) for i in range(N-K+1)]

# print(max(var))
# -> TLE

### 解説
### 尺取り法

colors = {C[i]: 0 for i in range(N)}

# 最初の区間を設定しておく
tmp = 0
for i in range(K):
    if colors[C[i]] == 0:
        tmp += 1
    colors[C[i]] += 1

ans = tmp
for i in range(N-K):
    # print(f"{colors} -> {tmp}")

    # 範囲の最初
    if colors[C[i]] == 1:  # 範囲の色の残りが1つだった時
        tmp -= 1
    colors[C[i]] -= 1  # 減らす

    # 範囲の最後
    if colors[C[i+K]] == 0:  # 範囲に色が存在しなかった時
        tmp += 1
    colors[C[i+K]] += 1  # 増やす
    ans = max(ans, tmp)

print(ans) 