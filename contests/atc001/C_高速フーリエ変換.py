#               C - 高速フーリエ変換              
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/atc001/tasks/fft_c
# ----------------------------------------

# FFT
# https://negligible.hatenablog.com/entry/2021/11/13/025822#FFT-Type-II

"""
# 方針

ans = \sum_{i=0}^N
"""

import numpy as np

# FFT Type I, with NumPy
# Caution! Original input data is overwritten and destroyed.
def FFT_1(f):
    N = len(f)
    if N > 1:
        n = N // 2  # Do not use '/', which gives float.
        i = np.arange(n)
        # Prepare complex sinusoid, rotation operator
        W = np.exp(-2j * np.pi * i / N)
        # Butterfly computation
        f_tmp = f[:n] - f[n:]
        f[:n] += f[n:]
        f[n:] = W * f_tmp
        # Recursively call this function itself
        FFT_1(f[:n])  # First half
        FFT_1(f[n:])  # Second half
        # Simple permutation
        F = np.empty_like(f)
        F[0::2] = f[:n]
        F[1::2] = f[n:]
        # Overwrite results to input
        f[:] = F

def IFFT_1(f, origin=1):
    N = len(f)
    if N > 1:
        n = N // 2  # Do not use '/', which gives float.
        i = np.arange(n)
        # Prepare complex sinusoid, rotation operator
        W = np.exp(2j * np.pi * i / N)
        # Butterfly computation
        f_tmp = f[:n] - f[n:]
        f[:n] += f[n:]
        f[n:] = W * f_tmp
        # Recursively call this function itself
        IFFT_1(f[:n], 0)  # First half
        IFFT_1(f[n:], 0)  # Second half
        # Simple permutation
        F = np.empty_like(f)
        F[0::2] = f[:n]
        F[1::2] = f[n:]
        # Overwrite results to input
        f[:] = F
        if origin:
            f /= N

if __name__ == "__main__":
    N = int(input())
    size = 1 << N.bit_length() + 1
    A = np.zeros(size, dtype='complex')
    B = np.zeros(size, dtype='complex')

    for i in range(N):
        A[i+1], B[i+1] = map(int, input().split())
    
    # FFT
    FFT_1(A)
    FFT_1(B)

    # 要素ごとの積
    C = np.array([a*b for a,b in zip(A,B)])

    # IFFT
    IFFT_1(C)

    # 実部を出力
    for c in C[1:2*N+1]:
        print(round(c.real))
