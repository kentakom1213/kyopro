
# WA
# from bisect import bisect_left

# N = int(input())
# A = list(map(int, input().split()))
# X = int(input())

# # 累積和
# lA = len(A)
# S = [sum(A[:i]) for i in range(1, lA+1)]
# sumA = S[-1]

# res_i = lA * (X // sumA)
# res = X % sumA

# i = bisect_left(S, res)

# print(res_i + i + 1)

# 改良
# AC

def binary_search(condition, min, max):
    while max - min > 1:
        mid = int((max + min) // 2)
        if condition(mid):
            min = mid
        else:
            max = mid
    return min

N = int(input())
A = list(map(int, input().split()))
X = int(input())

sA = sum(A)
lA = len(A)

res_i = lA * (X // sA)
res = X % sA

i = binary_search(lambda x: sum(A[0:x]) <= res, 0, lA)

print(res_i + i + 1)
