#        051 - Typical Shop（★5）
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/typical90/tasks/typical90_ay
# ----------------------------------------

# 半分全列挙？？

def binary_search(condition, min, max):
    while max - min > 1:
        mid = int((max + min) // 2)
        if condition(mid):
            min = mid
        else:
            max = mid
    return min


N, K, P = map(int, input().split())
A = [int(n) for n in input().split()]

# 半分ずつに分ける
U, V = A[:N//2], A[N//2:]
lu, lv = N//2, N-N//2

# Vの部分和をあらかじめ作っておく
p_sum_V = []
for i in range(1 << lv):
    p_sum = 0
    for j in range(lv):
        if (i>>j) & 1:
            p_sum += V[j]
    p_sum_V.append(p_sum)

p_sum_V.sort()

# Uの部分和
ans = 0
for i in range(1 << lu):
    p_sum_u = 0
    for j in range(lu):
        if (i>>j) & 1:
            p_sum_u += U[j]
    
    # p_sum_u + p_sum_v <= P を満たす組合せを探索
    comb = binary_search(
        lambda i: p_sum_u + p_sum_V[i] <= P,
        0,
        N
    )
    print(p_sum_u, comb)

print(ans)
