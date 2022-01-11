from bisect import bisect_left

N, K = map(int, input().split())
P = list(map(int, input().split()))

# 先頭K項をソート
top = sorted(P[:K])
print(top[-K])

# 挿入しつつ出力
for i in range(K, N):
    top.insert(bisect_left(top, P[i]), P[i])
    print(top[-K])
