#              C - Swappable
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc206/tasks/abc206_c

# 安易に組み込み関数を使わないこと

# AC
# ----------------------------------------

# N = int(input())
# A = list(map(int, input().split()))

# # おそらくボトルネック
# # O(n^2)
# dup = filter(lambda x: x>1, (A.count(n) for n in set(A)))

# def comb_2(n): return n * (n - 1) // 2

# # res =  comb(N, 2) - sum( comb(n, 2) | (n in dup) )
# res = comb_2(N) - sum( comb_2(n) for n in dup )
# print(res)

# # -> TLE

def comb_2(n): return n * (n - 1) // 2

N = int(input())
A = list(map(int, input().split()))

A.sort()

res = comb_2(N)
temp = 0
now = A[0]
for i in range(N+1):
    if i < N and now == A[i]:
        temp += 1
    else:
        res -= comb_2(temp)
        temp = 1
        if i < N: now = A[i]
    
print(res)