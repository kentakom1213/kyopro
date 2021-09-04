# D - Factorization
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/abc110/tasks/abc110_d

# 参考
# https://drken1215.hatenablog.com/entry/2018/09/23/224100
# https://drken1215.hatenablog.com/entry/2018/06/08/210000

# 初の 10^9+7 で割ったあまりを求める問題
# 一回で理解できない部分があまりに多いので後で復習する。

# AC
# ----------------------------------------

# まず素因数分解
def fac_count(n):
    result = []
    for i in range(2, int(n ** 0.5 + 1)):
        if n % i != 0: continue
        counter = 0
        while n % i == 0:
            n //= i
            counter += 1
        result.append((i, counter))
    if n != 1:
        result.append((n, 1))
    return result


# 二項係数の生成
# https://drken1215.hatenablog.com/entry/2018/06/08/210000
MAX = 210000  # 何を根拠に？
MOD = 1000000007
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


if __name__ == "__main__":
    N, M = map(int, input().split())

    COMinit()

    Mfactors = fac_count(M)

    res = 1
    for p, num in Mfactors:
        tmp = COM(num+N-1, N-1)
        res = (res * tmp) % MOD

    print(res)
