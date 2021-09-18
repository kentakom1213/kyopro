#  D - 表現の自由 ( Freedom of expression )
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/arc004/tasks/arc004_4

# MLE -> pypyでは厳しい？ -> Pythonにしたら解決
# AC
# ----------------------------------------


def factoring(n):
    max_num = round(n ** 0.5) + 1
    res = []
    for i in range(2, max_num+1):
        if n % i != 0: continue
        counter = 0
        while n % i == 0:
            counter += 1
            n //= i
        res.append((i, counter))
    if n != 1: res.append((n, 1))
    return res


### factorial, factorial-inverseの配列を用意
MAX = 320000  # N <= 10^9 より 
MOD = 1_000_000_007
fac, finv, inv = [0]*MAX, [0]*MAX, [0]*MAX

def COMinit():
    fac[0] = fac[1] = 1
    finv[0] = finv[1] = 1
    inv[1] = 1
    for i in range(2, MAX):
        fac[i] = fac[i - 1] * i % MOD
        inv[i] = MOD - inv[MOD % i] * (MOD // i) % MOD
        finv[i] = finv[i - 1] * inv[i] % MOD

def COM(n, k):
    if n < k: return 0
    if n < 0 or k < 0: return 0
    return fac[n] * (finv[k] * finv[n - k] % MOD) % MOD

def pow2_mod(n):
    res = 1
    for _ in range(n):
        res = res * 2 % MOD
    return res


if __name__ == "__main__":
    N, M = map(int, input().split())
    
    COMinit()

    factors = [n for p, n in factoring(abs(N))]

    res = 1
    for n in factors:
        temp = COM(n+M-1, n)  # nHr = n+r-1Cr
        res = (res * temp) % MOD
    
    # print(res)  # 符号なしであればこれで完成
    # 符号に関して
    # n個の整数で1を表す組み合わせをf(n)とおくと、
    # f(1) = 1
    # f(n) = 2 * f(n-1)
    # よって、 f(n) = 2^(n-1)
    # print(pow2_mod(N-1))

    print(res * pow2_mod(M-1) % MOD)