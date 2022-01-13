#           D - Sum of Divisors
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc172/tasks/abc172_d

# pythonだとTLE
# pypyだとセーフ

# AC
# ----------------------------------------

# 約数を求める計算量は O(sqrt(n))
# Kについてループを回すと O(n sqrt(n))
# N <= 10^7 --> 10^10 回くらいになってしまい、間に合わない

# 篩を使えばうまくいく？
# 計算量 O(n log(n))

if __name__ == "__main__":
    N = int(input())

    sieve = [0] * (N + 1)

    for i in range(1, N+1):
        for j in range(i, (N+1) // i + 1):
            if i*j <= N:
                if i == j: sieve[i*j] += 1
                else: sieve[i*j] += 2
    
    sieve[1] = 1
    
    res = 0
    for i, val in enumerate(sieve):
        res += i * val
    
    print(res)
