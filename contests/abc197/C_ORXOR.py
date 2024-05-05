#                C - ORXOR
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc197/tasks/abc197_c

# 参考
# https://ebisuke33.hatenablog.com/entry/abc197c

# 何言ってるかわからん

# AC (解説)
# ----------------------------------------

# 方針
# N <= 20 であるから、bit全探索 O(2^n) では 2^20 ~= 10^6 で間に合う
# bitの操作を学ぶ機会として

N = int(input())
A = list(map(int, input().split()))

res = 1e10

if len(A) == 1:
    print(A[0])
else:
    for bit in range(1 << (N-1)):
        xor = 0
        log_sum = A[0]

        for i in range(1, N):
            if (bit >> (i-1)) & 1:
                xor ^= log_sum
                log_sum = 0
                log_sum |= A[i]
            else:
                log_sum |= A[i]
        
        xor ^= log_sum
        res = min(res, xor)
    
    print(res)