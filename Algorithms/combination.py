
def comb(n, r):
    if r == 0: return 1
    else:
        return comb(n, r-1) * (n - r + 1) // r