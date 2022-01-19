# E - Amusement Park
# TLE

from bisect import bisect_left
def mapl(func, iter): return list(map(func, iter))

# input
N, K = map(int, input().split())
A = sorted(mapl(int, input().split()), reverse=True)

result = 0
while K > 0:
    diff = A[0] - A[1]
    if diff == 0:
        result += A[0] * diff - diff * (diff + 1) / 2
        K -= diff
    else:
        pass


print(result)

# # solve
# result = 0
# while K > 0 and A[-1] > 0:
#     result += A[-1]
#     A[-1] -= 1
#     # A.insert(bisect_left(A, a), a)
#     A.append(A.pop(-2))

#     K -= 1
#     # print(K, A, "->", result)  # test

