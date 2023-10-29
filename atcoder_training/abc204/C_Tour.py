#                 C - Tour
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc204/tasks/abc204_c

# 解説
# https://atcoder.jp/contests/abc204/editorial/1991

# 解説ACは本当に悔しい

# AC (解説)
# ----------------------------------------

# N, M = map(int, input().split())

# pathes = [[] for i in range(N)]
# for _ in range(M):
#     a, b = map(int, input().split())
#     pathes[a-1].append(b-1)

### 方針
# スタート、ゴールの組を列挙 O(n^2)
# 可能性を判定 O(n)
# -> O(n^3)

# @lru_cache
# def can_move(s, g):
#     if depth > N:
#         return False
#     if g in pathes[s]:
#         return True
#     else:
#         for p in pathes[s]:
#             return can_move(p, g, depth+1)

# counter = N
# for i in range(N):
#     for j in range(N):
#         if i != j and can_move(i, j):
#             counter += 1

# print(counter)
# # WA & TLE

# 解説
import sys
sys.setrecursionlimit(10000)

N, M = map(int, input().split())

pathes = [[] for i in range(N)]
for _ in range(M):
    a, b = map(int, input().split())
    pathes[a-1].append(b-1)


def can_move(v):
    if temp[v]: return
    temp[v] = True
    for vv in pathes[v]:
        can_move(vv)

ans = 0

for i in range(N):  # O(N)
    temp = [False] * N  # iからjに移動可能かどうかをメモ
    can_move(i)  # O(M+N)
    ans += sum(temp)

print(ans)
# -> O(N(M+N))