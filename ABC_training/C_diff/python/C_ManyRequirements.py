#         C - Many Requirements
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc165/tasks/abc165_c

# 解説
# https://blog.hamayanhamayan.com/entry/2020/05/02/225726

# C問題までは基本的に全探索！！
# pythonは神

# AC
# ----------------------------------------

# 同じ条件に関して、得点が大きいものを採用


# N, M, Q = map(int, input().split())

# queries = {}
# for _ in range(Q):
#     a, b, c, d = map(int, input().split())
#     a -= 1
#     b -= 1

#     now = queries[(a, b)] if (a, b) in queries else 0
#     queries[(a, b)] = max(now, d)

# # ソートして貪欲に採用
# q_sorted = sorted(queries.items(), key=lambda x: x[1], reverse=True)

# res = [0] * N
# for a, b, point in q_sorted:


# 解説
# 数列の全探索をする
# 全ての組み合わせは combi(10, N) であるから最大でも combi(10, 5) = 252
# それぞれに関して条件 (<= 50) をチェックしても十分間に合う

from itertools import combinations_with_replacement

N, M, Q = map(int, input().split())
queries = [tuple(map(int, input().split())) for _ in range(Q)]

nums = list(range(1, M+1))

max_point = 0

for A in combinations_with_replacement(nums, N):
    point = 0
    # 条件にマッチしているかをチェック
    for a, b, c, d in queries:
        a -= 1
        b -= 1

        if A[b] - A[a] == c:
            point += d
        
    max_point = max(max_point, point)

print(max_point)