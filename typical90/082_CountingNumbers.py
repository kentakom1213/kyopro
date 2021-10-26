#       082 - Counting Numbers（★3）
# ----------------------------------------
# 問題
# https://atcoder.jp/contests/typical90/tasks/typical90_cd
# ----------------------------------------

# L ~ R

# 4桁
# 1000 ~ 9999
# 4 * (9999 - 1000 + 1) (文字)

# n桁
# 10*(n-1) ~ 10*n-1
# n * (10*n - 10*(n-1))

# MOD = int(1e9 + 7)

# if __name__ == "__main__":
#     L, R = map(int, input().split())
#     ll, lr = len(str(L)), len(str(R))

#     res = 0
#     for n in range(ll, lr):
#         res += n * (10**n - 10**(n-1))
#         res %= MOD
    
#     res -= ll * (L - 10**ll)
#     res += lr * (R - 10**lr)

#     print(res)

# 難しい