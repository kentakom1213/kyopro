
def comb_re(n, r):
    r = min(r, n-r)
    if r == 0: return 1
    else:
        return comb(n, r-1) * (n - r + 1) // r

def comb(n, r):
    r = min(r, n-r)
    if r == 0: return 1
    else:
        res = 1
        for i in range(1, r+1):
            res *= (n - i + 1) / i
        return int(res)


if __name__ == "__main__":
    N = int(input())
    print(comb(10, N))
    print(comb_re(10, N))